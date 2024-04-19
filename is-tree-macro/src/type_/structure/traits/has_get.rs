use quote::quote;

use crate::type_::Structure;

pub(crate) fn impl_has_get(structure: &Structure) -> proc_macro2::TokenStream {
    if structure.implementation.has_branches && structure.implementation.has_path_segment {
        let structure_name = &structure.name;
        quote! {
            impl<'a> ::is_tree::HasGet<'a> for &'a #structure_name {}
            impl<'a> ::is_tree::HasGet<'a> for &'a mut #structure_name {}
    
            impl<'a> ::is_tree::HasGetOrCreate<'a> for #structure_name {}
        }    
    } else {
        Default::default()
    }
}