use crate::instructions::Instruction;
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream};
use syn::{parse_str, LitInt};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Var {
    Implied,
    Ident(Ident),
    Position(u8),
}

impl Var {
    pub fn add_span(&mut self, span: Span) {
        if let Var::Ident(i) = self {
            i.set_span(span)
        }
    }
}

impl Parse for Var {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Var::Implied)
        } else {
            match input.parse::<LitInt>() {
                Ok(l) => {
                    let pos: u8 = l
                        .base10_parse()
                        .map_err(|_| input.error("position must be between 0 and 255."))?;
                    if !input.is_empty() {
                        return Err(input.error("expected count."));
                    }
                    Ok(Var::Position(pos))
                }
                Err(_) => {
                    let res = input.parse::<Ident>().map(Var::Ident)?;
                    if !input.is_empty() {
                        return Err(input.error("expected identifier"));
                    }
                    Ok(res)
                }
            }
        }
    }
}

pub fn parse_var(input: String, input_span: Span) -> syn::Result<Instruction> {
    match input.split_once(':') {
        Some((var, split)) => {
            let mut var: Var = parse_str(var)?;
            var.add_span(input_span);
            if let Some((sep, num)) = split.rsplit_once(':') {
                if sep.is_empty() {
                    return Err(syn::Error::new(input_span, "separator cannot be empty."));
                }
                Ok(if num.trim().is_empty() {
                    if !cfg!(feature = "alloc") {
                        return Err(syn::Error::new(
                            input_span,
                            "alloc feature is required to parse into a Vec.",
                        ));
                    }
                    Instruction::VecParse(var, String::from(sep))
                } else {
                    match num.parse() {
                        Ok(0_u8) => Instruction::IterParse(var, String::from(sep)),
                        Ok(x) => Instruction::MultiParse(var, String::from(sep), x),
                        Err(_) => {
                            return Err(syn::Error::new(
                                input_span,
                                format!("expected a number between 0 and 255 but found {num}."),
                            ));
                        }
                    }
                })
            } else {
                Err(syn::Error::new(
                    input_span,
                    "invalid multi parse, it must be of the form <var>:<sep>:<count>.",
                ))
            }
        }
        None => {
            let mut var: Var = parse_str(&input)?;
            var.add_span(input_span);
            Ok(Instruction::Parse(var))
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;

    use crate::instructions::Instructions;

    #[test]
    fn test_instruction_pass() {
        use crate::instructions::Instruction::*;
        use crate::var::Var::*;
        #[rustfmt::skip]
        let cases = [
            ("{}", vec![Parse(Implied)]),
            ("{} {}", vec![Parse(Implied), Lit(" ".into()), Parse(Implied)]),
            ("{}\n{}", vec![Parse(Implied), Lit("\n".into()), Parse(Implied)]),
            ("😇{}ángeĺ{}!", vec![Lit("😇".into()), Parse(Implied), Lit("ángeĺ".into()), Parse(Implied), Lit("!".into())]),
            ("{}{{{}}}{}", vec![Parse(Implied), Lit("{".into()), Parse(Implied), Lit("}".into()), Parse(Implied)]),
            (" {}{{:}}}}{} ", vec![Lit(" ".into()), Parse(Implied), Lit("{:}}".into()), Parse(Implied), Lit(" ".into())]),
            (" {} {}}}{}", vec![Lit(" ".into()), Parse(Implied), Lit(" ".into()), Parse(Implied), Lit("}".into()), Parse(Implied)]),
            ("{:}}:}", vec![VecParse(Implied, "}".into())]),
            ("{:{{}}:}", vec![VecParse(Implied, "{}".into())]),
            ("{:{{}}: }", vec![VecParse(Implied, "{}".into())]),
            ("{hello}", vec![Parse(Ident(syn::Ident::new("hello", Span::call_site())))]),
            ("{:,:5}", vec![MultiParse(Implied, ",".into(), 5)]),
            ("{:,:0}", vec![IterParse(Implied, ",".into())]),
            ("{:,:}", vec![VecParse(Implied, ",".into())]),
            ("{:,::1}", vec![MultiParse(Implied, ",:".into(), 1)]),
            ("{:,::0}", vec![IterParse(Implied, ",:".into())]),
            ("{:,::}", vec![VecParse(Implied, ",:".into())]),
            ("{::,::85}", vec![MultiParse(Implied, ":,:".into(), 85)]),
            ("{::,::0}", vec![IterParse(Implied, ":,:".into())]),
            ("{::,::}", vec![VecParse(Implied, ":,:".into())]),
            ("{ 0  }", vec![Parse(Position(0))]),
            ("{1} {0}", vec![Parse(Position(1)), Lit(" ".into()), Parse(Position(0))]),
            ("{0} {  hiya }", vec![Parse(Position(0)), Lit(" ".into()), Parse(Ident(syn::Ident::new("hiya", Span::call_site())))]),
        ];
        for (input, expected) in cases {
            let output = Instructions::new(input, Span::call_site());
            assert_eq!(output.unwrap(), Instructions(expected));
        }
    }
}
