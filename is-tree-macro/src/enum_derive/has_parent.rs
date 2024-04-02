use proc_macro2::TokenStream;
use syn::DeriveInput;
use quote::quote;

pub fn impl_has_parent(ast: &DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let _self = quote! { #name #generics };

    let mut variants = quote!{};
    
    for variant in &data.variants {
        let variant_name = &variant.ident;
        variants = quote! {
            #variants
            #name::#variant_name(value) => value.parent().into(),
        };
    }
    
    quote! {
        impl<'a> ::is_tree::KnowsParent<'a> for #_self {
            type Parent = #_self;
        }

        impl<'a> ::is_tree::HasParent<'a> for &'a #_self {
            fn parent(self) -> Self::Parent {
                match self {
                    #variants
                }
            }
        }
    }
}