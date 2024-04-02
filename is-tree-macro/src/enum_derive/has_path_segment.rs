use proc_macro2::TokenStream;
use syn::DeriveInput;
use quote::quote;

pub fn impl_has_path_segment(ast: &DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };
    
    let mut path_segment_variants = quote!{};
    let mut path_variants = quote!{};

    for variant in &data.variants {
        let variant_name = &variant.ident;
        path_segment_variants = quote! {
            #path_segment_variants
            #name::#variant_name(value) => value.path_segment(),
        };
        path_variants = quote! {
            #path_variants
            #name::#variant_name(value) => value.path(),
        };
    }
    
    quote! {
        impl<'a> ::is_tree::HasPathSegment for #_self {
            fn path_segment(&self) -> &String {
                match self {
                    #path_segment_variants
                }
            }
        }

        impl<'a> ::is_tree::HasPath for #_self {
            fn path(&self) -> ::is_tree::Path {
                match self {
                    #path_variants
                }
            }
        }
    }
}