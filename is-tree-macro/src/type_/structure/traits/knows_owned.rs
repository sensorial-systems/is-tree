use quote::quote;

use crate::type_::Structure;

pub(crate) fn impl_knows_owned(structure: &Structure) -> proc_macro2::TokenStream {
    let structure_name = &structure.name;
    quote! {
        impl ::is_tree::KnowsOwned for #structure_name {
            type Owned = #structure_name;
        }
    }
}