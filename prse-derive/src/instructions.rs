use itertools::Itertools;
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

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Instruction {
    Lit(String),
    Parse(Var),
    VecParse(Var, String),
    IterParse(Var, String),
    MultiParse(Var, String, u8),
}

impl Instruction {
    pub(crate) fn get_var(&self) -> Option<&Var> {
        match self {
            Instruction::Lit(_) => None,
            Instruction::Parse(v)
            | Instruction::VecParse(v, _)
            | Instruction::IterParse(v, _)
            | Instruction::MultiParse(v, _, _) => Some(v),
        }
    }
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
                        "Found unexpected } bracket. Consider escaping it by changing it to }}.",
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
                if !matches!(instructions.last(), Some(Instruction::Lit(_)) | None) {
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
        return Err(syn::Error::new(
            input_span,
            "Expected to find } bracket. Consider adding a } bracket to close the open { bracket.",
        ));
    }
    if !val.is_empty() {
        instructions.push(Instruction::Lit(val));
    }

    validate_instructions(instructions, input_span)
}

fn validate_instructions(
    instructions: Vec<Instruction>,
    input_span: Span,
) -> syn::Result<Vec<Instruction>> {
    if instructions
        .iter()
        .any(|i| matches!(i.get_var(), Some(Var::Position(_))))
    {
        if !instructions
            .iter()
            .any(|i| matches!(i.get_var(), Some(Var::Implied)))
        {
            let has_constant_step = instructions
                .iter()
                .flat_map(|i| match i.get_var() {
                    Some(Var::Position(p)) => Some(p),
                    _ => None,
                })
                .sorted()
                .zip(0_u8..)
                .all(|(i, p)| i == &p);
            if has_constant_step {
                Ok(instructions)
            } else {
                Err(syn::Error::new(input_span, "Each positional argument much uniquely map to a corresponding index in the returned tuple."))
            }
        } else {
            Err(syn::Error::new(
                input_span,
                "Cannot use implied positional arguments with explicitly defined ones.",
            ))
        }
    } else {
        Ok(instructions)
    }
}

fn parse_var(input: String, input_span: Span) -> syn::Result<Instruction> {
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

    use crate::instructions::get_instructions;

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
            let output = get_instructions(input, Span::call_site());
            assert_eq!(output.unwrap(), expected);
        }
    }
}
