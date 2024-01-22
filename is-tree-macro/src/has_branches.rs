use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};
use quote::quote;

pub fn impl_has_branches(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };
    if let Data::Enum(data) = &ast.data {
        let mut variants = quote!{};

        for variant in &data.variants {
            let variant_name = &variant.ident;
            variants = quote! {
                #variants
                #name::#variant_name(value) => value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
            };
        }

        let variant = data.variants.first().expect("Enum must have at least one variant");
        let variant = variant.fields.iter().next().expect("Variant must have at least one field");

        let gat = quote! {
            <#variant as KnowsBranches<'a>>::Branches
        };
        
        quote! {
            impl<'a> ::is_tree::KnowsBranches<'a> for #_self {
                type Branches = #gat;
            }
    
            impl<'a> ::is_tree::HasBranches<'a> for #_self {
                fn branches(&'a self) -> impl Iterator<Item = Self::Branches> {
                    match self {
                            #variants
                    }
                }
            }
        }
    } else {
        quote!{}
    }
}