use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr, Token};

use crate::instructions::{get_instructions, Instruction, Var};

#[derive(Clone)]
pub struct ParseInvocation {
    pub input: Option<Expr>,
    pub instructions: Vec<Instruction>,
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
                            (__prse_parse, __prse_input) = __prse_input.split_once(#c).unwrap();
                        },
                        Err(_) => quote! {
                            (__prse_parse, __prse_input) = __prse_input.split_once(#l_string).unwrap();
                        }
                    });
                    if let Some(t) = store_token {
                        store_token = None;
                        result.append_all(t);
                    }
                }
                Instruction::Parse(var) => {
                    let var = match var {
                        Var::Implied => {
                            idents_to_return.push(format_ident!("__prse_{}", idx));
                            idents_to_return.last().unwrap()
                        }
                        Var::Ident(i) => i,
                    };
                    store_token = Some(quote! {
                        let #var = __prse_parse.parse().unwrap();
                    });
                }
                Instruction::VecParse(_, _) => {}
                Instruction::IterParse(_, _) => {}
                Instruction::MultiParse(_, _, _) => {}
            };
        }
        if let Some(t) = store_token {
            result.append_all(quote! {
                __prse_parse = __prse_input;
                #t
            })
        }
        let end = quote! {
            ( #(#idents_to_return),* )
        };
        result.append_all(end);

        tokens.append_all(quote! {
            {
                #result
            }
        })
    }
}
