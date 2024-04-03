use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_parent(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.parent().into(),
        }
    }).collect::<TokenStream>();
    
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
