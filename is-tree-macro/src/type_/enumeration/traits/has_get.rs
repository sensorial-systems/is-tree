use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_get(enumeration: &Enumeration) -> TokenStream {
    // let name = &enumeration.name;
    // let generics = &enumeration.generics;
    // let _self = quote! { #name #generics };
    // let variants = enumeration.variants.iter().map(|variant| {
    //     let variant_name = &variant.variant.ident;
    //     quote! {
    //         #name::#variant_name(value) => value.get(segment).map(|value| value.into()),
    //     }
    // }).collect::<TokenStream>();
    
    // quote! {
    //     impl<'a> ::is_tree::HasGet<'a> for &'a #_self {
    //         fn get(self, segment: impl Into<String>) -> Option<Self::Branches> {
    //             match self {
    //                 #variants
    //             }
    //         }
    //     }
    // }
    quote!{}
}