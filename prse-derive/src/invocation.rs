use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, LitStr, Token};

use crate::instructions::{get_instructions, Instruction, Var};

#[derive(Clone)]
pub struct ParseInvocation {
    input: Expr,
    instructions: Vec<Instruction>,
    pub try_parse: bool,
}

impl Parse for ParseInvocation {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let input = stream.parse()?;
        let _coma: Token![,] = stream.parse()?;
        let lit = stream.parse::<LitStr>()?;
        let lit_string = lit.value();
        let instructions = get_instructions(&lit_string, lit.span())?;

        Ok(Self {
            input,
            try_parse: false,
            instructions,
        })
    }
}

impl ParseInvocation {
    fn gen_function(&self, body: TokenStream, tokens: &mut TokenStream) {
        let mut generics = vec![];
        let mut return_types = vec![];
        for (idx, i) in self
            .instructions
            .iter()
            .enumerate()
            .filter(|(_, i)| !matches!(i, Instruction::Lit(_)))
        {
            let type_ident = format_ident!("T{idx}");
            generics.push(match i {
                Instruction::Parse(_) => type_ident.to_token_stream(),
                Instruction::VecParse(_, _) => {
                    if cfg!(feature = "std") {
                        quote!(::std::vec::Vec<#type_ident>)
                    } else {
                        quote!(::alloc::vec::Vec<#type_ident>)
                    }
                }
                Instruction::IterParse(_, _) => quote! {
                    impl ::core::iter::Iterator<Item = ::core::result::Result<#type_ident, ::prse::ParseError>> + 'a
                },
                Instruction::MultiParse(_, _, count) => {
                    let count = *count as usize;
                    quote! ([ #type_ident ; #count])
                }
                _ => unreachable!(),
            });
            return_types.push(type_ident);
        }

        tokens.append_all(quote! {
            fn __prse_func<'a, #(#return_types: LendingFromStr<'a>),* >(
                mut __prse_input: &'a str,
            ) -> ::core::result::Result<( #(#generics),* ), ::prse::ParseError> {
                #body
            }
        })
    }

    fn gen_body(&self, result: &mut TokenStream) {
        let mut store_token = None;
        let alloc_crate: TokenStream = if cfg!(feature = "std") {
            quote!(std)
        } else {
            quote!(alloc)
        };

        for (idx, i) in self.instructions.iter().enumerate() {
            let var = format_ident!("__prse_{idx}");
            match i {
                Instruction::Lit(l_string) => {
                    let l_string = string_to_tokens(l_string);

                    result.append_all(if cfg!(feature = "alloc") {
                        quote! {
                            (__prse_parse, __prse_input) = __prse_input.split_once(#l_string)
                                .ok_or_else(|| ::prse::ParseError::Literal {expected: (#l_string).into(), found: __prse_input.into()})?;
                        }
                    } else {
                        quote! {
                            (__prse_parse, __prse_input) = __prse_input.split_once(#l_string)
                                .ok_or_else(|| ::prse::ParseError::Literal)?;
                        }
                    });

                    if let Some(t) = store_token {
                        store_token = None;
                        result.append_all(t);
                    }
                }
                Instruction::Parse(_) => {
                    store_token = Some(quote! {
                        let #var = __prse_parse.lending_parse()?;
                    });
                }
                Instruction::VecParse(_, sep) => {
                    let sep = string_to_tokens(sep);
                    store_token = Some(quote! {
                        let #var = __prse_parse.split(#sep)
                            .map(|p| p.lending_parse())
                            .collect::<::core::result::Result<::#alloc_crate::vec::Vec<_>, ::prse::ParseError>>()?;
                    });
                }
                Instruction::IterParse(_, sep) => {
                    let sep = string_to_tokens(sep);
                    store_token = Some(quote! {
                        let #var = __prse_parse.split(#sep)
                            .map(|p| p.lending_parse());
                    });
                }
                Instruction::MultiParse(_, sep, count) => {
                    let sep = string_to_tokens(sep);
                    let i = 0..*count;
                    store_token = Some(quote! {
                        let mut __prse_iter = __prse_parse.split(#sep)
                            .map(|p| p.lending_parse());
                        let #var = [ #(
                            __prse_iter.next()
                            .ok_or_else(|| ::prse::ParseError::Multi {
                                expected: #count,
                                found: #i,
                            })??
                        ),* ];
                        if __prse_iter.next().is_some() {
                            return Err(::prse::ParseError::Multi {
                                expected: #count,
                                found: #count + 1,
                            });
                        }
                    });
                }
            };
        }
        result.append_all(store_token.map_or_else(|| if cfg!(feature = "alloc") {
            quote! {
                if !__prse_input.is_empty() {
                    return Err(::prse::ParseError::Literal {expected: "".into(), found: __prse_input.into()})
                }
            }
        } else {
            quote! {
                if !__prse_input.is_empty() {
                    return Err(::prse::ParseError::Literal)
                }
            }
        }, |t| quote! { __prse_parse = __prse_input; #t }));

        let return_idents = self.instructions.iter().enumerate().filter_map(|(idx, i)| {
            i.get_var()?;
            Some(format_ident!("__prse_{idx}"))
        });
        result.append_all(quote! { Ok(( #(#return_idents),* )) });
    }
}

impl ToTokens for ParseInvocation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut renames = TokenStream::new();
        let mut return_idents = vec![];
        let mut func_idents = vec![];
        let mut num_positions = 0;

        for (idx, instruction) in self
            .instructions
            .iter()
            .enumerate()
            .filter(|(_, i)| !matches!(i, Instruction::Lit(_)))
        {
            let var = instruction.get_var().unwrap();
            let ident = format_ident!("__prse_{idx}");
            match var {
                Var::Implied => {
                    func_idents.push(ident.clone());
                    return_idents.push(ident);
                }
                Var::Ident(i) => {
                    func_idents.push(ident.clone());
                    renames.append_all(quote!(#i = #ident;));
                }
                Var::Position(p) => {
                    func_idents.push(format_ident!("__prse_pos_{p}"));
                    return_idents.push(format_ident!("__prse_pos_{num_positions}"));
                    num_positions += 1;
                }
            };
        }

        let mut body = quote!(let mut __prse_parse;);
        let mut function = quote!(
            use ::prse::{ExtParseStr, LendingFromStr};
        );

        self.gen_body(&mut body);

        let func_idents: Vec<_> = self
            .instructions
            .iter()
            .enumerate()
            .filter_map(|(idx, i)| {
                Some(match i.get_var()? {
                    Var::Position(p) => format_ident!("__prse_pos_{p}"),
                    _ => format_ident!("__prse_{idx}"),
                })
            })
            .collect();

        self.gen_function(body, &mut function);

        let input = &self.input;

        let mut result = quote_spanned! { input.span() =>
            #[allow(clippy::needless_borrow)]
            let mut __prse_input: &str = &#input;
        };

        result.append_all(if self.try_parse {
            quote! {
                match __prse_func(__prse_input) {
                    Ok(( #(#func_idents),* )) => {
                        #renames
                        Ok(( #(#return_idents),* ))
                    }
                    Err(e) => Err(e),
                }
            }
        } else {
            quote! {
                let ( #(#func_idents),* ) = __prse_func(__prse_input).unwrap();
                #renames
                ( #(#return_idents),* )
            }
        });

        tokens.append_all(quote! {
            {
                #function

                #result
            }
        });
    }
}

fn string_to_tokens(string: &str) -> TokenStream {
    string
        .parse()
        .map_or_else(|_| string.to_token_stream(), |s: char| s.to_token_stream())
}
