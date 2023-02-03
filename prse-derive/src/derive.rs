use crate::instructions::{get_instructions, Instruction, Var};
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use std::collections::HashSet;
use syn::parse::{Parse, ParseStream};
use syn::{
    Attribute, Data, DeriveInput, Generics, Lit, LitStr, Meta, MetaNameValue, Type, Variant,
};

#[derive(Clone)]
pub(crate) enum Derive {
    NoAttributes(Generics, Ident),
    Struct(Generics, Ident, Fields),
    Enum(Generics, Ident, Vec<(Ident, Fields)>),
}

#[derive(Clone)]
pub(crate) enum Fields {
    Named(Vec<(Ident, Type)>, Vec<Instruction>),
    Unnamed(Vec<Type>, Vec<Instruction>),
    Unit(Instruction),
}

fn validate_fields(
    fields: syn::Fields,
    instructions: Vec<Instruction>,
    span: Span,
) -> syn::Result<Fields> {
    match fields {
        syn::Fields::Unit => {
            let mut iter = instructions.into_iter();
            match iter.next() {
                None => Ok(Fields::Unit(Instruction::Lit("".into()))),
                Some(Instruction::Lit(s)) if iter.next().is_none() => {
                    Ok(Fields::Unit(Instruction::Lit(s)))
                }
                _ => Err(syn::Error::new(
                    span,
                    "A unit field cannot contain variables",
                )),
            }
        }
        syn::Fields::Named(fields) => {
            let fields: Vec<_> = fields
                .named
                .into_iter()
                .map(|f| (f.ident.unwrap(), f.ty))
                .collect();
            let mut seen_idents = HashSet::new();
            for i in instructions.iter() {
                match i.get_var() {
                    None => {}
                    Some(Var::Ident(ident)) => {
                        if fields.iter().any(|(i, _)| i == ident) {
                            if seen_idents.contains(ident) {
                                return Err(syn::Error::new(
                                    span,
                                    format!("Duplicated variable: {ident}"),
                                ));
                            }
                            seen_idents.insert(ident);
                        } else {
                            return Err(syn::Error::new(
                                span,
                                format!("Unexpected variable: {ident}"),
                            ));
                        }
                    }
                    _ => {
                        return Err(syn::Error::new(
                            span,
                            "Named fields can only be parsed by name.",
                        ));
                    }
                }
            }
            Ok(Fields::Named(fields, instructions))
        }
        syn::Fields::Unnamed(fields) => {
            let max = fields.unnamed.iter().count() - 1;
            let mut count = 0;
            for i in instructions.iter() {
                match i.get_var() {
                    Some(Var::Ident(ident)) => {
                        return Err(syn::Error::new(
                            span,
                            format!("Unexpected named variable: {ident}."),
                        ));
                    }
                    Some(Var::Implied) => {
                        if count > max {
                            return Err(syn::Error::new(
                                span,
                                format!("Tuple variable must be between 0 and {max}."),
                            ));
                        }
                        count += 1;
                    }
                    Some(Var::Position(pos)) if (*pos as usize) > max => {
                        return Err(syn::Error::new(
                            span,
                            format!("Positional variable must be between 0 and {max}."),
                        ));
                    }
                    _ => {}
                }
            }
            Ok(Fields::Unnamed(
                fields.unnamed.into_iter().map(|f| f.ty).collect(),
                instructions,
            ))
        }
    }
}

impl Parse for Derive {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let input: DeriveInput = stream.parse()?;

        match input.data {
            Data::Struct(s) => {
                no_attributes(s.fields.iter().flat_map(|f| f.attrs.iter()))?;
                match attribute_instructions(input.attrs.into_iter())? {
                    None => Ok(Derive::NoAttributes(input.generics, input.ident)),
                    Some((instructions, span)) => Ok(Derive::Struct(
                        input.generics,
                        input.ident,
                        validate_fields(s.fields, instructions, span)?,
                    )),
                }
            }
            Data::Enum(e) => {
                no_attributes(input.attrs.iter())?;
                no_attributes(
                    e.variants
                        .iter()
                        .flat_map(|v| v.fields.iter().flat_map(|f| f.attrs.iter())),
                )?;

                match get_variant_attributes(e.variants.into_iter(), input.ident.span())? {
                    None => Ok(Derive::NoAttributes(input.generics, input.ident)),
                    Some(v_instructions) => {
                        Ok(Derive::Enum(input.generics, input.ident, v_instructions))
                    }
                }
            }
            Data::Union(u) => Err(syn::Error::new(
                u.union_token.span,
                "The derive macro does not currently support unions.",
            )),
        }
    }
}

fn attribute_instructions(
    mut attrs: impl Iterator<Item = Attribute>,
) -> syn::Result<Option<(Vec<Instruction>, Span)>> {
    while let Some(a) = attrs.next() {
        if let Some(lit) = get_prse_lit(&a)? {
            for a in attrs.by_ref() {
                if get_prse_lit(&a)?.is_some() {
                    return Err(syn::Error::new(
                        a.bracket_token.span,
                        "Expected only a single prse attribute.",
                    ));
                }
            }
            dbg!(&a.tokens);
            let span = lit.span();
            let lit_string = lit.value();
            return Ok(Some((get_instructions(&lit_string, span)?, span)));
        }
    }
    Ok(None)
}

fn get_prse_lit(a: &Attribute) -> syn::Result<Option<LitStr>> {
    if a.path.is_ident("prse") {
        match a.parse_meta()? {
            Meta::NameValue(MetaNameValue {
                lit: Lit::Str(l), ..
            }) => Ok(Some(l)),
            _ => Err(syn::Error::new(
                a.bracket_token.span,
                "prse attribute must be of the form #[prse = \"parse_string\"]",
            )),
        }
    } else {
        Ok(None)
    }
}

fn no_attributes<'a>(attrs: impl Iterator<Item = &'a Attribute>) -> syn::Result<()> {
    attrs.fold(Ok(()), |i, a| {
        let error = syn::Error::new(a.bracket_token.span, "Unexpected prse attribute.");
        let error = match get_prse_lit(a).map(|a| a.is_some()) {
            Err(mut e) => {
                e.combine(error);
                e
            }
            Ok(true) => error,
            Ok(false) => {
                return i;
            }
        };
        match i {
            Ok(()) => Err(error),
            Err(mut e) => {
                e.combine(error);
                Err(e)
            }
        }
    })
}

fn get_variant_attributes(
    iter: impl Iterator<Item = Variant>,
    input_span: Span,
) -> syn::Result<Option<Vec<(Ident, Fields)>>> {
    iter.map(|v| {
        (
            (v.ident, v.fields),
            attribute_instructions(v.attrs.into_iter()),
        )
    })
    .try_fold(
        Some(vec![]),
        |i, ((v_ident, v_fields), instructions)| match i {
            Some(mut v) if v.is_empty() => match instructions? {
                None => Ok(None),
                Some((instr, span)) => {
                    v.push((v_ident, validate_fields(v_fields, instr, span)?));
                    Ok(Some(v))
                }
            },
            Some(mut v) => match instructions? {
                None => Err(syn::Error::new(
                    input_span,
                    "The derive macro must either have an attribute on each field or none at all.",
                )),
                Some((instr, span)) => {
                    v.push((v_ident, validate_fields(v_fields, instr, span)?));
                    Ok(Some(v))
                }
            },
            None => match instructions? {
                None => Ok(None),
                Some(_) => Err(syn::Error::new(
                    input_span,
                    "The derive macro must either have an attribute on each field or none at all.",
                )),
            },
        },
    )
}

pub(crate) fn _expand_derive(_input: DeriveInput) -> TokenStream {
    // match input.data
    todo!()
}
//
// fn expand_field(fields: FieldsNamed) -> TokenStream {}
//
// fn expand_tuple(fields: FieldsUnnamed) -> TokenStream {}
//
// fn expand_unit() -> TokenStream {}

fn _expand_default(mut generics: Generics, name: Ident) -> TokenStream {
    let ty_generics = generics.split_for_impl().1.to_token_stream();

    generics.params.push(parse_quote!('__prse_a));
    let predicates = &mut generics.make_where_clause().predicates;
    predicates.push(parse_quote!(Self: ::core::str::FromStr));
    predicates.push(parse_quote!(
        <Self as ::core::str::FromStr>::Err: core::convert::Into<::prse::ParseError>
    ));
    let (impl_generics, _, where_clause) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::prse::LendingFromStr<'__prse_a> for #name #ty_generics #where_clause {
            fn from_str(s: &'__prse_a str) -> Result<Self, ::prse::ParseError> {
                <Self as ::core::str::FromStr>::from_str(&s).map_err(|e| e.into())
            }
        }
    }
}
