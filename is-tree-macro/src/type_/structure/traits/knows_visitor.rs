use quote::quote;

use crate::{traits::AttributeQuery, type_::Structure};

pub fn impl_knows_visitor(structure: &Structure) -> proc_macro2::TokenStream {
    if let Some(value) = structure.named_attribute_value(vec!["tree", "visitor"]) {
        let name = &structure.name;
        quote! {
            impl<'a> ::is_tree::KnowsVisitor<'a> for #name {
                type RelativeType = #value;
            }

            impl<'a> ::is_tree::KnowsVisitor<'a> for &'a #name {
                type RelativeType = #value;
            }
        }
    } else {
        quote! {}
    }
}
