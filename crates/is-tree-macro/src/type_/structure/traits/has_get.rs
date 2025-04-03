use quote::quote;

use crate::type_::Structure;

pub(crate) fn impl_has_get(structure: &Structure) -> proc_macro2::TokenStream {
    let structure_name = &structure.name;
    quote! {
        impl<'a> ::is_tree::HasGetMut<'a> for #structure_name {
            fn get_mut<T>(&'a mut self, segment: impl Into<String>) -> Option<T>
            where &'a mut Self: ::is_tree::HasGet + ::is_tree::HasBranches<T>,
                  T: ::is_tree::HasPathSegment + 'a
            {
                use ::is_tree::HasGet;
                self.get_impl::<T>(segment)
            }        
        }

        // impl ::is_tree::
    }
}