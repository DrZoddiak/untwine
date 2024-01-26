use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    punctuated::Punctuated,
    token::{Gt, Lt},
    AngleBracketedGenericArguments, Path, PathArguments, PathSegment, Type, TypePath,
};

use crate::{Modifier, Pattern, PatternFragment};

pub fn optional(typ: Type) -> Type {
    let tokens: TokenStream = quote! {
        Option<#typ>
    }
    .into();
    syn::parse(tokens).unwrap()
}

pub fn vec_of(typ: Type) -> Type {
    let tokens: TokenStream = quote! {
        Vec<#typ>
    }
    .into();
    syn::parse(tokens).unwrap()
}

fn fragment_type(fragment: &PatternFragment, parser_types: &HashMap<String, Type>) -> Type {
    let tokens = match fragment {
        crate::PatternFragment::Literal(_) => quote! {&str},
        crate::PatternFragment::CharRange(_) => quote! {char},
        crate::PatternFragment::ParserRef(ident) => {
            return parser_types[&ident.to_string()].clone()
        }
        crate::PatternFragment::Labeled(pat) => return fragment_type(&pat.pattern, parser_types),
        crate::PatternFragment::Nested(_) => todo!(),
    };
    syn::parse(tokens.into()).unwrap()
}

pub fn get_type(pattern: Pattern, parser_types: &HashMap<String, Type>) -> Type {
    let typ = fragment_type(&pattern.fragment, parser_types);
    match pattern.modifier {
        Some(Modifier::Optional) => optional(typ),
        Some(Modifier::Repeating | Modifier::OptionalRepeating) => vec_of(typ),
        None => typ,
    }
}