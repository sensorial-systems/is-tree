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
                #name::#variant_name(value) => value.get(key).map(|value| value.into()),
            };
        }

        let variant = data.variants.first().expect("Enum must have at least one variant");
        let variant = variant.fields.iter().next().expect("Variant must have at least one field");

        let gat = quote! {
            <#variant as KnowsGetType>::GetType
        };
        
        quote! {
            impl<'a> ::is_tree::KnowsGetType for #_self {
                type GetType = #gat;
            }
    
            impl<'a> ::is_tree::HasGet for #_self {
                fn get<K>(&self, key: K) -> Option<Self::GetType>
                where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
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