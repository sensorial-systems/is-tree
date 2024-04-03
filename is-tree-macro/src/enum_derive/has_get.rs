use proc_macro2::TokenStream;
use syn::DeriveInput;
use quote::quote;

pub fn impl_has_get(ast: &DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };
    let mut variants = quote!{};

    for variant in &data.variants {
        let variant_name = &variant.ident;
        variants = quote! {
            #variants
            #name::#variant_name(value) => value.get(segment).map(|value| value.into()),
        };
    }
    
    quote! {
        impl<'a> ::is_tree::HasGet<'a> for &'a #_self {
            fn get(self, segment: impl Into<String>) -> Option<Self::Branches> {
                match self {
                    #variants
                }
            }
        }
    }
}