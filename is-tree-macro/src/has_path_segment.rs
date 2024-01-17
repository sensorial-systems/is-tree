use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};
use quote::quote;

pub fn impl_has_path_segment(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };
    
    if let Data::Enum(data) = &ast.data {
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

        let variant = data.variants.first().expect("Enum must have at least one variant");
        let variant = variant.fields.iter().next().expect("Variant must have at least one field");

        let gat = quote! {
            <#variant as KnowsPathSegment>::PathSegment
        };
        
        quote! {
            impl<'a> ::is_tree::KnowsPathSegment for #_self {
                type PathSegment = #gat;
            }
    
            impl<'a> ::is_tree::HasPathSegment for #_self {
                fn path_segment(&self) -> &Self::PathSegment {
                    match self {
                        #path_segment_variants
                    }
                }
            }

            impl<'a> ::is_tree::HasPath for #_self {
                fn path(&self) -> ::is_tree::Path<Self::PathSegment> {
                    match self {
                        #path_variants
                    }
                }
            }
        }
    } else {
        quote!{}
    }
}