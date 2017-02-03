#![recursion_limit = "128"]

extern crate proc_macro;

extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Variant};
use quote::Tokens;

#[proc_macro_derive(FromStr)]
pub fn derive_from_str(input: TokenStream) -> TokenStream {
    let input = match syn::parse_derive_input(&input.to_string()) {
        Ok(tokens) => tokens,
        Err(msg) => panic!("Internal error from `syn`: {}", msg),
    };
    match expand_derive_from_str(&input) {
        Ok(expanded) => match expanded.parse() {
            Ok(parsed) => parsed,
            Err(msg) => panic!("Internal error from `quote`: {:?}", msg),
        },
        Err(msg) => panic!("{}", msg),
    }
}

fn expand_derive_from_str(input: &DeriveInput) -> Result<Tokens, String> {

    let variants = check_unitary_enum(input)?;
    let mut conditions = vec![];

    for variant in variants {
        let ref reified = variant.ident;
        let ref str = &reified.to_string();
        conditions.push(quote!{
            if #str == s {
                return Ok(#reified);
            }
        })
    }

    let ref name = input.ident;
    let dummy_const = syn::Ident::new(
        "_IMPL_FROM_STR_FOR_".to_string() + &(name.to_string()).to_uppercase()
    );

    Ok(quote!{
        const #dummy_const: () = {
            extern crate std;
            use self::#name::*;
            impl std::str::FromStr for #name {
                type Err = String;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    #(#conditions)*
                    return Err(format!("No variants are named {} in the enum.", s))
                }
            }
        };
    })

}

#[proc_macro_derive(Variants)]
pub fn derive_variants(input: TokenStream) -> TokenStream {
    let input = match syn::parse_derive_input(&input.to_string()) {
        Ok(tokens) => tokens,
        Err(msg) => panic!("Internal error from `syn`: {}", msg),
    };
    match expand_derive_variants(&input) {
        Ok(expanded) => match expanded.parse() {
            Ok(parsed) => parsed,
            Err(msg) => panic!("Internal error from `quote`: {:?}", msg),
        },
        Err(msg) => panic!("{}", msg),
    }
}

fn expand_derive_variants(input: &DeriveInput) -> Result<Tokens, String> {

    let variants = check_unitary_enum(input)?;
    let mut names_of_variants = vec![];

    for variant in variants {
        names_of_variants.push(variant.ident.to_string())
    }

    let idents_of_variants = names_of_variants.into_iter().map(|s| syn::Ident::new(s));
    let ref name = input.ident;
    let dummy_const = syn::Ident::new(
        "_IMPL_VARIANTS_FOR_".to_string() + &(name.to_string()).to_uppercase()
    );

    Ok(quote!{
        const #dummy_const: () = {
            use enum_variants::Variants;
            impl Variants for #name {
                fn variants() -> Vec<Self> {
                    use self::#name::*;
                    vec![#(#idents_of_variants),*]
                }
            }
        };
    })

}

fn check_unitary_enum(input: &DeriveInput) -> Result<&[Variant], String> {
    match input.body {
        syn::Body::Enum(ref variants) => {
            for variant in variants {
                if variant.data != syn::VariantData::Unit {
                    return Err("Variants must be unitary.".into());
                }
            }
            Ok(variants)
        },
        syn::Body::Struct(_) => Err("Cannot derive `Variants` for a struct.".into())
    }
}
