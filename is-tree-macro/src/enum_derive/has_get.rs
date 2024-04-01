use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};
use quote::quote;

pub fn impl_has_get(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };
    if let Data::Enum(data) = &ast.data {
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
                fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::Branches>
                where PathSegment: Into<String>
                {
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