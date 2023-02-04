use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, LitStr, Token};

use crate::instructions::Instructions;

#[derive(Clone)]
pub struct ParseInvocation {
    input: Expr,
    instructions: Instructions,
    pub try_parse: bool,
}

impl Parse for ParseInvocation {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let input = stream.parse()?;
        let _coma: Token![,] = stream.parse()?;
        let lit = stream.parse::<LitStr>()?;
        let lit_string = lit.value();
        let instructions = Instructions::new(&lit_string, lit.span())?;

        Ok(Self {
            input,
            try_parse: false,
            instructions,
        })
    }
}

impl ParseInvocation {}

impl ToTokens for ParseInvocation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let func_name = format_ident!("__prse_func");
        let input = &self.input;
        let mut renames = vec![];
        let mut return_idents = vec![];
        let mut func_idents = vec![];

        self.instructions
            .gen_return_idents(&mut return_idents, &mut func_idents, &mut renames);

        let renames: TokenStream = renames.iter().flat_map(|(l, r)| quote!(#l = #r;)).collect();

        let mut body = quote!(let mut __prse_parse: &str;);

        self.instructions.gen_body(&mut body);

        let function = self.instructions.gen_function(body, func_name.clone());

        let mut result = quote_spanned! { input.span() =>
            #[allow(clippy::needless_borrow)]
            let mut __prse_input: &str = &#input;
        };

        result.append_all(if self.try_parse {
            quote! {
                match #func_name (__prse_input) {
                    Ok(( #(#func_idents),* )) => {
                        #renames
                        Ok(( #(#return_idents),* ))
                    }
                    Err(e) => Err(e),
                }
            }
        } else {
            quote! {
                let ( #(#func_idents),* ) = ::prse::__private::unwrap_parse( #func_name (__prse_input), __prse_input);
                #renames
                #[allow(clippy::unused_unit)]
                {
                    ( #(#return_idents),* )
                }
            }
        });

        tokens.append_all(quote! {
            {
                use ::prse::{ExtParseStr, LendingFromStr};

                #function

                #result
            }
        });
    }
}

pub(crate) fn string_to_tokens(string: &str) -> TokenStream {
    string
        .parse()
        .map_or_else(|_| string.to_token_stream(), |s: char| s.to_token_stream())
}
