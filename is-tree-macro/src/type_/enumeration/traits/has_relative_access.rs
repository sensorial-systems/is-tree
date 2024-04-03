use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_relative_access(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };
    
    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.relative(path),
        }
    }).collect::<TokenStream>();

    let variant = enumeration.variants.first().map(|first| &first.variant).expect("Enum must have at least one variant");
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
}