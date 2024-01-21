use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};
use quote::quote;

pub fn impl_has_root(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };
    
    if let Data::Enum(data) = &ast.data {
        let mut variants = quote!{};

        for variant in &data.variants {
            let variant_name = &variant.ident;
            variants = quote! {
                #variants
                #name::#variant_name(value) => value.root(),
            };
        }

        let variant = data.variants.first().expect("Enum must have at least one variant");
        let variant = variant.fields.iter().next().expect("Variant must have at least one field");

        let gat = quote! {
            <#variant as KnowsRoot<'a>>::Root
        };
        
        quote! {
            impl<'a> ::is_tree::KnowsRoot<'a> for #_self {
                type Root = #gat;
            }
    
            impl<'a> ::is_tree::HasRoot<'a> for #_self {
                fn root(&self) -> Self::Root {
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