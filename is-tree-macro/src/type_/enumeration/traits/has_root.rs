use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_root(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.root(),
        }
    }).collect::<TokenStream>();

    let variant = enumeration.variants.first().map(|first| &first.variant).expect("Enum must have at least one variant");
    let variant = variant.fields.iter().next().expect("Variant must have at least one field");

    let gat = quote! {
        <#variant as KnowsRoot<'a>>::Root
    };
    
    quote! {
        impl<'a> ::is_tree::KnowsRoot<'a> for #_self {
            type Root = #gat;
        }

        impl<'a> ::is_tree::HasRoot<'a> for &'a #_self {
            fn root(self) -> Self::Root {
                match self {
                    #variants
                }
            }
        }
    }
}