use proc_macro2::TokenStream;
use quote::quote;

use crate::{traits::AttributeQuery, type_::Enumeration};

pub fn impl_knows_branches(enumeration: &Enumeration) -> TokenStream {
    let structure_name = &enumeration.name;
    let branches = enumeration
        .named_attribute_value(vec!["tree", "branches"])
        .unwrap_or_else(|| structure_name.clone().into());
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");
    quote! {
        impl<'a> ::is_tree::KnowsBranches<'a> for #reference  {
            type Branches = #branches;
        }

        impl<'a> ::is_tree::KnowsBranches<'a> for &'a #reference {
            type Branches = #branches;
        }
    }
}

pub fn impl_has_branches(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => longer_ref(value).branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
        }
    }).collect::<TokenStream>();

    quote! {
        impl<'a> ::is_tree::HasBranches<'a> for &'a #reference {
            fn branches(self) -> impl Iterator<Item = Self::Branches> {
                fn longer_ref<'longer, T>(t: &T) -> &T { t }
                match self {
                    #variants
                }
            }
        }

        impl<'a> ::is_tree::HasBranches<'a> for #reference {
            fn branches(self) -> impl Iterator<Item = Self::Branches> {
                #[inline]
                fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
                match &self {
                    #variants
                }
            }
        }
    }
}

pub fn impl_branches(enumeration: &Enumeration) -> TokenStream {
    let knows_branches = impl_knows_branches(enumeration);
    let has_branches = impl_has_branches(enumeration);
    quote! {
        #knows_branches
        #has_branches
    }
}