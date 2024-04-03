use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_branches(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
        }
    }).collect::<TokenStream>();

    let variant = enumeration.variants.first().map(|first| &first.variant).expect("Enum must have at least one variant");
    let variant = variant.fields.iter().next().expect("Variant must have at least one field");

    let gat = quote! {
        <#variant as KnowsBranches<'a>>::Branches
    };
    
    quote! {
        impl<'a> ::is_tree::KnowsBranches<'a> for #_self {
            type Branches = #gat;
        }

        impl<'a> ::is_tree::KnowsBranches<'a> for &'a #_self {
            type Branches = #gat;
        }

        impl<'a> ::is_tree::KnowsBranches<'a> for &'a mut #_self {
            type Branches = #gat;
        }

        impl<'a> ::is_tree::HasBranches<'a> for &'a #_self {
            fn branches(self) -> impl Iterator<Item = Self::Branches> {
                match self {
                    #variants
                }
            }
        }
    }
}