use crate::invocation::string_to_tokens;
use crate::var;
use crate::var::Var;
use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt};

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Instruction {
    Lit(String),
    Parse(Var),
    VecParse(Var, String, bool),
    IterParse(Var, String, bool),
    MultiParse(Var, String, u8, bool),
}

impl Instruction {
    pub(crate) fn get_var(&self) -> Option<&Var> {
        match self {
            Instruction::Lit(_) => None,
            Instruction::Parse(v)
            | Instruction::VecParse(v, ..)
            | Instruction::IterParse(v, ..)
            | Instruction::MultiParse(v, ..) => Some(v),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub(crate) struct Instructions(pub Vec<Instruction>);

impl Instructions {
    pub fn new(input: &str, input_span: Span) -> syn::Result<Instructions> {
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
                    instructions.push(var::parse_var(val, input_span)?);
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

        Self::validate_instructions(instructions, input_span)
    }

    fn validate_instructions(
        instructions: Vec<Instruction>,
        input_span: Span,
    ) -> syn::Result<Instructions> {
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
                    Ok(Instructions(instructions))
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
            Ok(Instructions(instructions))
        }
    }

    pub fn gen_function(&self, body: TokenStream, func_name: Ident) -> TokenStream {
        let mut return_types = vec![];
        let mut generics = vec![];
        for (idx, i) in self
            .0
            .iter()
            .enumerate()
            .filter(|(_, i)| !matches!(i, Instruction::Lit(_)))
        {
            let type_ident = format_ident!("T{idx}");
            return_types.push(match i {
                Instruction::Parse(_) => type_ident.to_token_stream(),
                Instruction::VecParse(..) => {
                    if cfg!(feature = "std") {
                        quote!(::std::vec::Vec<#type_ident>)
                    } else {
                        quote!(::alloc::vec::Vec<#type_ident>)
                    }
                }
                Instruction::IterParse(..) => quote! {
                   ::prse::ParseIter<'a, #type_ident>
                },
                Instruction::MultiParse(_, _, count, _) => {
                    let count = *count as usize;
                    quote! ([ #type_ident ; #count])
                }
                _ => unreachable!(),
            });
            generics.push(type_ident);
        }

        quote! {
            fn #func_name <'a, #(#generics: Parse<'a>),* >(
                mut __prse_input: &'a str,
            ) -> ::core::result::Result<( #(#return_types),* ), ::prse::ParseError> {
                #body
            }
        }
    }

    pub fn gen_body(&self, result: &mut TokenStream) {
        let mut store_token = None;
        let alloc_crate: TokenStream = if cfg!(feature = "std") {
            quote!(std)
        } else {
            quote!(alloc)
        };

        for (idx, i) in self.0.iter().enumerate() {
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
                Instruction::VecParse(_, sep, is_multi) => {
                    store_token = Some(quote! {
                        let #var = ::prse::ParseIter::new(__prse_parse, #sep, #is_multi)
                            .collect::<::core::result::Result<::#alloc_crate::vec::Vec<_>, ::prse::ParseError>>()?;
                    });
                }
                Instruction::IterParse(_, sep, is_multi) => {
                    store_token = Some(quote! {
                        let #var = ::prse::ParseIter::new(__prse_parse, #sep, #is_multi);
                    });
                }
                Instruction::MultiParse(_, sep, count, is_multi) => {
                    let i = 0..*count;
                    store_token = Some(quote! {
                        let mut __prse_iter = ::prse::ParseIter::new(__prse_parse, #sep, #is_multi);
                        let #var = [ #(
                            __prse_iter.next()
                            .ok_or_else(|| ::prse::ParseError::Multi {
                                expected: #count,
                                found: #i,
                            })??
                        ),* ];
                        let __prse_count_left = __prse_iter.count();
                        if __prse_count_left != 0 {
                            return Err(::prse::ParseError::Multi {
                                expected: #count,
                                found: #count + __prse_count_left as u8,
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

        let return_idents = self.0.iter().enumerate().filter_map(|(idx, i)| {
            i.get_var()?;
            Some(format_ident!("__prse_{idx}"))
        });
        result.append_all(quote! { Ok(( #(#return_idents),* )) });
    }

    pub fn gen_return_idents(
        &self,
        return_idents: &mut Vec<Ident>,
        func_idents: &mut Vec<Ident>,
        renames: &mut Vec<(Ident, Ident)>,
    ) {
        let mut num_positions = 0;

        for (idx, instruction) in self
            .0
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
                    renames.push((i.clone(), ident));
                }
                Var::Position(p) => {
                    func_idents.push(format_ident!("__prse_pos_{p}"));
                    return_idents.push(format_ident!("__prse_pos_{num_positions}"));
                    num_positions += 1;
                }
            };
        }
    }
}
