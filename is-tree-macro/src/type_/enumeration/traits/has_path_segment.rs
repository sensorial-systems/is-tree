use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_path_segment(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };
    let path_segment_variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.path_segment(),
        }
    }).collect::<TokenStream>();

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.path(),
        }
    }).collect::<TokenStream>();
    quote! {
        impl #generics ::is_tree::HasPathSegment for #self_ {
            fn path_segment(&self) -> &String {
                match self {
                    #path_segment_variants
                }
            }
        }

        impl #generics ::is_tree::HasPath for #self_ {
            fn path(&self) -> ::is_tree::Path {
                match self {
                    #variants
                }
            }
        }
    }
}
