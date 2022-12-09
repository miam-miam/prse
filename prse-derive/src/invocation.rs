use crate::instructions::{get_instructions, Instruction};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, LitStr, Token};

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
