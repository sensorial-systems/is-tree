use proc_macro2::TokenStream;
use quote::quote;

use crate::{traits::AttributeQuery, type_::Enumeration};

pub fn impl_has_parent(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");
    let visitor = enumeration
        .named_attribute_value(vec!["tree", "visitor"])
        .expect("#[tree(visitor = \"type\")] not found in the enumeration.");

    let variants = enumeration.variants.iter().map(|variant| {
        let variant_name = &variant.variant.ident;
        quote! {
            #name::#variant_name(value) => value.parent().into(),
        }
    }).collect::<TokenStream>();
    
    quote! {
        impl<'a> ::is_tree::KnowsParent<'a> for #reference {
            type Parent = #visitor;
        }

        impl<'a> ::is_tree::HasParent<'a> for &'a #reference {
            fn parent(self) -> Self::Parent {
                match self {
                    #variants
                }
            }
        }

        // TODO: Implement mutable version
    }
}
