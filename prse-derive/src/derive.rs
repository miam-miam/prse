use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields, Generics};

pub(crate) enum Derive {
    NoAttributes(Generics, Ident),
    Struct(Generics, Ident, Attribute, Fields),
    Enum(Generics, Ident, Vec<Variant>),
}

pub(crate) struct Variant {
    ident: Ident,
    fields: Fields,
    attribute: Attribute,
}

impl Parse for Derive {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        let input: DeriveInput = stream.parse()?;

        let f_attributes: Vec<Option<Attribute>> = match &input.data {
            Data::Struct(s) => s
                .fields
                .iter()
                .map(|f| get_attribute(f.attrs.iter()))
                .collect::<Result<_, _>>(),
            Data::Enum(e) => e
                .variants
                .iter()
                .map(|f| get_attribute(f.attrs.iter()))
                .collect::<Result<_, _>>(),
            Data::Union(u) => Err(syn::Error::new(
                (u.union_token).span(),
                "The derive macro does not currently support unions.",
            )),
        }?;

        if !f_attributes.iter().map(|o| o.is_some()).all_equal() {
            return Err(syn::Error::new(
                input.span(),
                "The derive macro must either have an attribute on each field or none at all.",
            ));
        }

        let attribute = get_attribute(input.attrs.iter())?;

        let flatten_attributes = f_attributes.first().unwrap_or(&None);

        match (input.data, attribute, flatten_attributes) {
            (_, None, None) => Ok(Derive::NoAttributes(input.generics, input.ident)),
            (Data::Struct(s), Some(a), None) => {
                Ok(Derive::Struct(input.generics, input.ident, a, s.fields))
            }
            (Data::Struct(s), _, Some(_)) => Err(syn::Error::new(
                s.fields.span(),
                "The derive macro does not support field attributes in a struct.",
            )),
            (Data::Enum(s), None, Some(_)) => Ok(Derive::Enum(
                input.generics,
                input.ident,
                f_attributes
                    .into_iter()
                    .zip(s.variants.into_iter())
                    .map(|(a, v)| Variant {
                        ident: v.ident,
                        fields: v.fields,
                        attribute: a.unwrap(),
                    })
                    .collect(),
            )),
            (Data::Enum(_), Some(a), _) => Err(syn::Error::new(
                a.span(),
                "The derive macro does not support single attributes in enums.",
            )),
            (Data::Union(_), _, _) => unreachable!(),
        }

        // for a in f_attributes {
        //     return Err(syn::Error::new(
        //         input.span(),
        //         "The derive macro must either have an attribute on each field or none at all.",
        //     ));
        // }
        //
        // match get_attribute(&input.attrs) {
        //     None => {}
        //     Some(attrs) => {
        //         if let Some(Some(a)) = f_attributes.first() {
        //             return Err(syn::Error::new(a.span(), "Unexpected attribute"));
        //         }
        //     }
        // }
        // //
        // // Ok(Self {
        // //     input,
        // //     try_parse: false,
        // //     instructions,
        // // })
        // todo!()
    }
}

fn get_attribute<'a>(attrs: impl Iterator<Item = &'a Attribute>) -> syn::Result<Option<Attribute>> {
    let mut attrs = attrs.filter(|a| {
        if let Ok(syn::Meta::NameValue(name_value)) = a.parse_meta() {
            name_value.path.is_ident("prse")
        } else {
            false
        }
    });

    match attrs.next() {
        None => Ok(None),
        Some(a) => match attrs.next() {
            Some(a) => Err(syn::Error::new(
                a.span(),
                "Expected only a single prse attribute.",
            )),
            None => Ok(Some(a.clone())),
        },
    }
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
