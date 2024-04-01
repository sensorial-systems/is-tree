use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};
use quote::quote;

pub fn impl_has_relative_access(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };
    
    if let Data::Enum(data) = &ast.data {
        let mut variants = quote!{};

        for variant in &data.variants {
            let variant_name = &variant.ident;
            variants = quote! {
                #variants
                #name::#variant_name(value) => value.relative(path),
            };
        }

        let variant = data.variants.first().expect("Enum must have at least one variant");
        let variant = variant.fields.iter().next().expect("Variant must have at least one field");

        let gat = quote! {
            <#variant as KnowsRelativeAccessType<'a>>::RelativeType
        };
        
        quote! {
            impl<'a> ::is_tree::KnowsRelativeAccessType<'a> for #_self {
                type RelativeType = #gat;
            }
    
            impl<'a> ::is_tree::HasRelativeAccess<'a> for &'a #_self {
                fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
                where K: Into<String>
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