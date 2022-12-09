use itertools::Itertools;
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseStream};
use syn::parse_str;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Var {
    Implied,
    Ident(Ident),
}

impl Parse for Var {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Var::Implied)
        } else {
            input.parse().map(Var::Ident)
        }
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Instruction {
    Lit(String),
    Parse(Var),
    VecParse(Var, String),
    IterParse(Var, String),
    MultiParse(Var, String, u8),
}

pub fn get_instructions(input: &str, input_span: Span) -> syn::Result<Vec<Instruction>> {
    let mut i = input.chars().multipeek();
    let mut var_mode = false;
    let mut val = String::new();
    let mut instructions = vec![];
    while let Some(c) = i.next() {
        match (c, var_mode) {
            ('{', false) => {
                // Character has been escaped.
                if let Some('{') = i.peek() {
                    val.push(c);
                    i.next().unwrap();
                } else {
                    if !val.is_empty() {
                        instructions.push(Instruction::Lit(val));
                    }
                    val = String::new();
                    var_mode = true;
                }
            }
            ('}', false) => {
                if let Some('}') = i.peek() {
                    val.push(c);
                    i.next().unwrap();
                } else {
                    return Err(syn::Error::new(
                        input_span,
                        "Unescaped }, consider changing to }}.",
                    ));
                }
            }
            ('{', true) => {
                if let Some('{') = i.peek() {
                    val.push(c);
                    i.next().unwrap();
                } else {
                    return Err(syn::Error::new(
                        input_span,
                        "Unescaped {, consider changing to {{.",
                    ));
                }
            }
            ('}', true) => {
                if let Some('}') = i.peek() {
                    if i.peek() != Some(&'}') {
                        val.push(c);
                        i.next().unwrap();
                        continue;
                    }
                }
                if let Some(Instruction::Lit(_)) | None = instructions.last() {
                } else {
                    return Err(syn::Error::new(
                        input_span,
                        "Cannot have two captures without a string in between.",
                    ));
                }
                instructions.push(parse_var(val, input_span)?);
                val = String::new();
                var_mode = false;
            }
            (c, _) => val.push(c),
        }
    }
    if var_mode {
        return Err(syn::Error::new(input_span, "Unclosed {."));
    }
    if !val.is_empty() {
        instructions.push(Instruction::Lit(val));
    }
    Ok(instructions)
}

fn parse_var(input: String, input_span: Span) -> syn::Result<Instruction> {
    match input.split_once(':') {
        Some((var, split)) => {
            let var = parse_str(var)?;
            let (sep, num) = split.rsplit_once(':').ok_or_else(|| {
                syn::Error::new(
                    input_span,
                    "When specifying a multi parse, it must be of the form :<sep>:<count>.",
                )
            })?;

            if sep.is_empty() {
                return Err(syn::Error::new(input_span, "Seperator cannot be empty."));
            }

            Ok(if num.trim().is_empty() {
                Instruction::VecParse(var, String::from(sep))
            } else {
                match num.parse().map_err(|_| {
                    syn::Error::new(
                        input_span,
                        format!("Expected a count between 0 and 255 but found {num}."),
                    )
                })? {
                    0_u8 => Instruction::IterParse(var, String::from(sep)),
                    x => Instruction::MultiParse(var, String::from(sep), x),
                }
            })
        }
        None => Ok(Instruction::Parse(parse_str(&input)?)),
    }
}

#[cfg(test)]
mod tests {
    use crate::instructions::get_instructions;
    use proc_macro2::Span;

    #[test]
    fn test_instruction_pass() {
        use super::Instruction::*;
        use super::Var::*;
        #[rustfmt::skip]
        let cases = [
            ("{}", vec![Parse(Implied)]),
            ("{} {}", vec![Parse(Implied), Lit(" ".into()), Parse(Implied)]),
            ("{}\n{}", vec![Parse(Implied), Lit("\n".into()), Parse(Implied)]),
            ("😇{}ángeĺ{}!", vec![Lit("😇".into()), Parse(Implied), Lit("ángeĺ".into()), Parse(Implied), Lit("!".into())]),
            ("{}{{{}}}{}", vec![Parse(Implied), Lit("{".into()), Parse(Implied), Lit("}".into()), Parse(Implied)]),
            (" {}{{:}}}}{} ", vec![Lit(" ".into()),Parse(Implied), Lit("{:}}".into()), Parse(Implied), Lit(" ".into())]),
            (" {} {}}}{}", vec![Lit(" ".into()),Parse(Implied), Lit(" ".into()), Parse(Implied), Lit("}".into()), Parse(Implied)]),
            ("{:}}:}", vec![VecParse(Implied, "}".into())]),
            ("{:{{}}:}", vec![VecParse(Implied, "{}".into())]),
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
        ];
        for (input, expected) in cases {
            let output = get_instructions(input, Span::call_site());
            assert_eq!(output.unwrap(), expected);
        }
    }
}
