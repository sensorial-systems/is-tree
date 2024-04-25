use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_parent(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.parent().into(),
        }
    }).collect::<TokenStream>();
    
    quote! {
        impl #generics ::is_tree::KnowsParent<'a> for #self_ {
            type Parent = #self_;
        }

        impl #generics ::is_tree::HasParent<'a> for &'a #self_ {
            fn parent(self) -> Self::Parent {
                match self {
                    #variants
                }
            }
        }

        // TODO: Implement mutable version
    }
}
