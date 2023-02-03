use crate::instructions::{get_instructions, Instruction};
use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Generics, LitStr, Type, Variant};

pub(crate) enum Derive {
    NoAttributes(Generics, Ident),
    Struct(Generics, Ident, Fields),
    Enum(Generics, Ident, Vec<(Ident, Fields)>),
}

pub(crate) enum Fields {
    Named(Vec<(Ident, Type)>, Vec<Instruction>),
    Unnamed(Vec<Type>, Vec<Instruction>),
    Unit(Instruction),
}

fn validate_fields(fields: syn::Fields, instructions: Vec<Instruction>) -> syn::Result<Fields> {
    todo!()
}

impl Parse for Derive {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let input: DeriveInput = stream.parse()?;
        let input_span = input.span();

        match input.data {
            Data::Struct(s) => {
                no_attributes(s.fields.iter().flat_map(|f| f.attrs.iter()))?;
                match attribute_instructions(input.attrs.into_iter())? {
                    None => Ok(Derive::NoAttributes(input.generics, input.ident)),
                    Some(instructions) => Ok(Derive::Struct(
                        input.generics,
                        input.ident,
                        validate_fields(s.fields, instructions)?,
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

                match get_variant_attributes(e.variants.into_iter(), input_span)? {
                    None => Ok(Derive::NoAttributes(input.generics, input.ident)),
                    Some(v_instructions) => {
                        Ok(Derive::Enum(input.generics, input.ident, v_instructions))
                    }
                }
            }
            Data::Union(_) => Err(syn::Error::new(
                input_span,
                "The derive macro does not currently support unions.",
            )),
        }
    }
}

fn attribute_instructions(
    attrs: impl Iterator<Item = Attribute>,
) -> syn::Result<Option<Vec<Instruction>>> {
    let mut attrs = attrs.filter(is_prse_attribute);

    match attrs.next() {
        None => Ok(None),
        Some(a) => match attrs.next() {
            Some(a) => Err(syn::Error::new(
                a.span(),
                "Expected only a single prse attribute.",
            )),
            None => {
                let lit = syn::parse2::<LitStr>(a.tokens)?;
                let lit_string = lit.value();
                Ok(Some(get_instructions(&lit_string, lit.span())?))
            }
        },
    }
}

fn is_prse_attribute(a: &Attribute) -> bool {
    if let Ok(syn::Meta::NameValue(name_value)) = a.parse_meta() {
        name_value.path.is_ident("prse")
    } else {
        false
    }
}

fn no_attributes<'a>(attrs: impl Iterator<Item = &'a Attribute>) -> syn::Result<()> {
    attrs.filter(|a| is_prse_attribute(a)).fold(Ok(()), |i, a| {
        let error = syn::Error::new(a.span(), "Expected only a single prse attribute.");
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
                Some(instr) => {
                    v.push((v_ident, validate_fields(v_fields, instr)?));
                    Ok(Some(v))
                }
            },
            Some(mut v) => match instructions? {
                None => Err(syn::Error::new(
                    input_span,
                    "The derive macro must either have an attribute on each field or none at all.",
                )),
                Some(instr) => {
                    v.push((v_ident, validate_fields(v_fields, instr)?));
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

pub(crate) fn expand_derive(input: DeriveInput) -> TokenStream {
    // match input.data
    todo!()
}
//
// fn expand_field(fields: FieldsNamed) -> TokenStream {}
//
// fn expand_tuple(fields: FieldsUnnamed) -> TokenStream {}
//
// fn expand_unit() -> TokenStream {}

fn expand_default(mut generics: Generics, name: Ident) -> TokenStream {
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
