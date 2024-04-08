use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;
use crate::traits::AttributeQuery;

pub fn impl_has_get(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let _self = quote! { #name #generics };
    let reference = enumeration
        .named_attribute_value(vec!["tree", "reference"])
        .expect("#[tree(reference = \"type\")] not found in the enumeration.");
    
    quote! {
        impl<'a> ::is_tree::HasGet<'a> for &'a #reference {}
    }
}