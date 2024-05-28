use quote::quote;

use crate::{traits::AttributeQuery, type_::Structure};

pub fn impl_knows_relative_type(structure: &Structure) -> proc_macro2::TokenStream {
    if let Some(value) = structure.named_attribute_value(vec!["tree", "relative_visitor"]).first() {
        let name = &structure.name;
        quote! {
            impl<'a> ::is_tree::KnowsRelativeAccess<'a> for #name {
                type RelativeAccess = #value;
            }
        }
    } else {
        quote! {}
    }
}
