mod instructions;
mod invocation;

use invocation::ParseInvocation;

extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ParseInvocation);
    input.to_token_stream().into()
}

#[proc_macro]
pub fn try_parse(input: TokenStream) -> TokenStream {
    input
}
