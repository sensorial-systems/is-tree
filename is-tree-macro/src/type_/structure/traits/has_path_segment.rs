use quote::quote;

use crate::{traits::AttributeQuery, type_::Structure};

pub(crate) fn impl_has_path_segment(structure: &Structure) -> proc_macro2::TokenStream {
    let structure_name = &structure.name;
    structure.fields.iter().filter(|field| field.has_attribute(vec!["tree", "path_segment"])).map(|field| {
        let field_name = &field.field.ident;
        quote! {
            impl ::is_tree::HasPathSegment for #structure_name {
                fn path_segment(&self) -> &String {
                    &self.#field_name
                }
            }
        }
    }).collect()
}