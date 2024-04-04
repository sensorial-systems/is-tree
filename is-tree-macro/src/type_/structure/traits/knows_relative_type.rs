use quote::quote;

use crate::{traits::AttributeQuery, type_::Structure};



pub fn impl_knows_relative_type(structure: &Structure) -> proc_macro2::TokenStream {
    if let Some(value) = structure.named_attribute_value(vec!["tree", "visitor"]) {
        let name = &structure.name;
        quote! {
            impl<'a> ::is_tree::KnowsRelativeAccessType<'a> for #name {
                type RelativeType = #value;
            }
        }
    } else {
        quote! {}
    }
}