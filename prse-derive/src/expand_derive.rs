use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{GenericParam, Generics, ImplGenerics, WhereClause, WherePredicate};

use crate::derive::{Derive, Fields};
use crate::instructions::Instructions;
use crate::invocation::string_to_tokens;

impl Derive {
    pub fn into_token_stream(self) -> TokenStream {
        match self {
            Derive::NoAttributes(g, i) => expand_default(g, i),
            Derive::Struct(mut g, name, f) => {
                let (impl_generics, ty_generics, where_clause) =
                    split_for_impl(&mut g, [].into_iter());

                let tokens = match f {
                    Fields::Named(instructions) => expand_field(instructions, quote!(Self), None),
                    Fields::Unnamed(instructions) => expand_tuple(instructions, quote!(Self), None),
                    Fields::Unit(s) => expand_unit(s, quote!(Self), None),
                };

                quote! {
                    #[automatically_derived]
                    impl #impl_generics ::prse::LendingFromStr<'__prse_a> for #name #ty_generics #where_clause {
                        fn from_str(s: &'__prse_a str) -> Result<Self, ::prse::ParseError> {
                            #tokens
                        }
                    }
                }
            }
            Derive::Enum(mut g, name, v) => {
                let (impl_generics, ty_generics, where_clause) =
                    split_for_impl(&mut g, [].into_iter());

                let mut result = None;

                for (variant, f) in v.into_iter().rev() {
                    result = Some(match f {
                        Fields::Named(instructions) => {
                            expand_field(instructions, quote!(Self::#variant), result)
                        }
                        Fields::Unnamed(instructions) => {
                            expand_tuple(instructions, quote!(Self::#variant), result)
                        }
                        Fields::Unit(s) => expand_unit(s, quote!(Self::#variant), result),
                    });
                }

                quote! {
                    #[automatically_derived]
                    impl #impl_generics ::prse::LendingFromStr<'__prse_a> for #name #ty_generics #where_clause {
                        fn from_str(s: &'__prse_a str) -> Result<Self, ::prse::ParseError> {
                            #result
                        }
                    }
                }
            }
        }
    }
}

fn expand_field(
    instructions: Instructions,
    to_return: TokenStream,
    error: Option<TokenStream>,
) -> TokenStream {
    let func_name = format_ident!("__prse_func");
    let mut renames = vec![];
    let mut return_idents = vec![];
    let mut func_idents = vec![];
    let error = error.unwrap_or_else(|| quote!(Err(e)));

    instructions.gen_return_idents(&mut return_idents, &mut func_idents, &mut renames);

    let mut body = quote!(let mut __prse_parse;);

    instructions.gen_body(&mut body);

    let function = instructions.gen_function(body, func_name.clone());

    let fields = renames.iter().map(|(l, r)| quote!(#l: #r));

    quote! {
        {
            use ::prse::{ExtParseStr, LendingFromStr};

            #function

            match #func_name (s) {
                Ok(( #(#func_idents),* )) => {
                    Ok(( #to_return { #(#fields),* }))
                }
                Err(e) => #error,
            }
        }
    }
}

fn expand_tuple(
    instructions: Instructions,
    to_return: TokenStream,
    error: Option<TokenStream>,
) -> TokenStream {
    let func_name = format_ident!("__prse_func");
    let mut _renames = vec![];
    let mut return_idents = vec![];
    let mut func_idents = vec![];
    let error = error.unwrap_or_else(|| quote!(Err(e)));

    instructions.gen_return_idents(&mut return_idents, &mut func_idents, &mut _renames);

    let mut body = quote!(let mut __prse_parse;);

    instructions.gen_body(&mut body);

    let function = instructions.gen_function(body, func_name.clone());

    quote! {
        {
            use ::prse::{ExtParseStr, LendingFromStr};

            #function

            match #func_name (s) {
                Ok(( #(#func_idents),* )) => {
                    Ok(( #to_return ( #(#return_idents),* )))
                }
                Err(e) => #error,
            }
        }
    }
}

fn expand_unit(s: String, to_return: TokenStream, error: Option<TokenStream>) -> TokenStream {
    let l_string = string_to_tokens(&s);
    let error = error.unwrap_or_else(|| {
        if cfg!(feature = "alloc") {
            quote! {
                Err(::prse::ParseError::Literal {expected: (#l_string).into(), found: s.into()})
            }
        } else {
            quote!(Err(::prse::ParseError::Literal))
        }
    });
    quote! {
        match s {
            #l_string => Ok(#to_return),
            _ => #error
        }
    }
}

fn expand_default(mut generics: Generics, name: Ident) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = split_for_impl(
        &mut generics,
        [
            parse_quote!(Self: ::core::str::FromStr),
            parse_quote!(
                <Self as ::core::str::FromStr>::Err: core::convert::Into<::prse::ParseError>
            ),
        ]
        .into_iter(),
    );

    quote! {
        #[automatically_derived]
        impl #impl_generics ::prse::LendingFromStr<'__prse_a> for #name #ty_generics #where_clause {
            fn from_str(s: &'__prse_a str) -> Result<Self, ::prse::ParseError> {
                <Self as ::core::str::FromStr>::from_str(&s).map_err(|e| e.into())
            }
        }
    }
}

fn split_for_impl(
    generics: &mut Generics,
    extra_predicates: impl IntoIterator<Item = WherePredicate>,
) -> (ImplGenerics, TokenStream, Option<&WhereClause>) {
    let ty_generics = generics.split_for_impl().1.to_token_stream();

    generics.params.push(parse_quote!('__prse_a));

    let type_predicates: Vec<WherePredicate> = generics
        .params
        .iter()
        .filter_map(|p| {
            if let GenericParam::Type(t) = p {
                Some(parse_quote!(#t: ::prse::LendingFromStr<'__prse_a>))
            } else {
                None
            }
        })
        .collect();

    let predicates = &mut generics.make_where_clause().predicates;
    predicates.extend(extra_predicates);
    predicates.extend(type_predicates.into_iter());

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    (impl_generics, ty_generics, where_clause)
}
