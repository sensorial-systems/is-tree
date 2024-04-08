use proc_macro2::TokenStream;
use quote::quote;

use crate::{traits::AttributeQuery, type_::Enumeration};

fn impl_has_path_segment(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");
    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.path_segment(),
        }
    }).collect::<TokenStream>();
    quote! {
        impl<'a> ::is_tree::HasPathSegment for #reference {
            fn path_segment(&self) -> &String {
                match self {
                    #variants
                }
            }
        }
    }
}

fn impl_has_path(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");
    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.path(),
        }
    }).collect::<TokenStream>();
    quote! {
        impl<'a> ::is_tree::HasPath for #reference {
            fn path(&self) -> ::is_tree::Path {
                match self {
                    #variants
                }
            }
        }
    }
}

pub fn impl_path_segment(enumeration: &Enumeration) -> TokenStream {
    let has_path_segment = impl_has_path_segment(enumeration);
    let has_path = impl_has_path(enumeration);
    quote! {
        #has_path_segment
        #has_path
    }
}