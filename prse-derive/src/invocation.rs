use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr, Token};

use crate::instructions::{get_instructions, Instruction};

#[derive(Clone)]
pub struct ParseInvocation {
    pub input: Option<Expr>,
    pub instructions: Vec<Instruction>,
    pub try_parse: bool,
}

impl Parse for ParseInvocation {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let input = if stream.peek(syn::LitStr) {
            // Bother with std vs non std later
            None
        } else {
            let i = stream.parse()?;
            let _coma: Token![,] = stream.parse()?;
            Some(i)
        };
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

impl ToTokens for ParseInvocation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let input = self.input.clone().unwrap();
        let mut result = TokenStream::new();
        let start = quote! {
            let mut __prse_input: &str = &#input;
            let mut __prse_parse;
        };
        let mut idents_to_return = vec![];
        result.append_all(start);
        let mut store_token = None;
        for (idx, i) in self.instructions.iter().enumerate() {
            match i {
                Instruction::Lit(l_string) => {
                    result.append_all(match l_string.parse() {
                        Ok::<char, _>(c) => quote! {
                            (__prse_parse, __prse_input) = __prse_input.split_once(#c)
                                .ok_or_else(|| ParseError::Literal {expected: (#c).into(), found: __prse_input.into()})?;
                        },
                        Err(_) => quote! {
                            (__prse_parse, __prse_input) = __prse_input.split_once(#l_string)
                                .ok_or_else(|| ParseError::Literal {expected: (#l_string).into(), found: __prse_input.into()})?;
                        }
                    });
                    if let Some(t) = store_token {
                        store_token = None;
                        result.append_all(t);
                    }
                }
                Instruction::Parse(var) => {
                    let var = var.get_ident(&mut idents_to_return, idx);
                    store_token = Some(quote! {
                        #var = __prse_parse.lending_parse()?;
                    });
                }
                Instruction::VecParse(var, sep) => {
                    let var = var.get_ident(&mut idents_to_return, idx);
                    let sep = sep
                        .parse()
                        .map_or_else(|_| sep.to_token_stream(), |s: char| s.to_token_stream());
                    store_token = Some(quote! {
                        #var = __prse_parse.split(#sep)
                            .map(|p| p.lending_parse())
                            .collect::<Result<Vec<_>, ParseError>>()?;
                    });
                }
                Instruction::IterParse(var, sep) => {
                    let var = var.get_ident(&mut idents_to_return, idx);
                    let sep = sep
                        .parse()
                        .map_or_else(|_| sep.to_token_stream(), |s: char| s.to_token_stream());
                    store_token = Some(quote! {
                        #var = __prse_parse.split(#sep)
                            .map(|p| p.lending_parse());
                    });
                }
                Instruction::MultiParse(var, sep, count) => {
                    let var = var.get_ident(&mut idents_to_return, idx);
                    let sep = sep
                        .parse()
                        .map_or_else(|_| sep.to_token_stream(), |s: char| s.to_token_stream());
                    let iterator_token = quote! {
                        let mut __prse_iter = __prse_parse.split(#sep)
                            .map(|p| p.lending_parse());
                    };
                    let i = 0..*count;
                    store_token = Some(quote! {
                        #iterator_token
                        #var = [ #(
                            __prse_iter.next()
                            .ok_or_else(|| ParseError::Multi {
                                expected: #count,
                                found: #i,
                            })??
                        ),* ];
                    });
                }
            };
        }
        if let Some(t) = store_token {
            result.append_all(quote! { __prse_parse = __prse_input; #t })
        } else {
            result.append_all(quote! {
                if !__prse_input.is_empty() {
                    return Err(ParseError::Literal {expected: "".into(), found: __prse_input.into()})
                }
            })
        }

        result.append_all(quote! { Ok::<_, ParseError>(( #(#idents_to_return),* )) });

        let function = if self.try_parse {
            quote!(__prse_func())
        } else {
            quote!(__prse_func().unwrap())
        };

        tokens.append_all(quote! {
            {
                use prse::*;
                let mut __prse_func = || {
                    #result
                };
                #function
            }
        });
    }
}
