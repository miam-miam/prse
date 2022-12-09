mod instructions;
mod invocation;

extern crate proc_macro2;
// #[macro_use]
// extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn try_parse(input: TokenStream) -> TokenStream {
    input
}
