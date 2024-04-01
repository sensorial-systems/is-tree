// mod has_parent;
// mod has_root;
mod has_path_segment;
mod has_branches;
mod has_get;
mod knows_owned;
// mod has_relative_access;

use quote::quote;

pub fn impl_struct(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let has_path_segment = has_path_segment::impl_has_path_segment(&ast);
    let has_branches = has_branches::impl_has_branches(&ast);
    let has_get = has_get::impl_has_get(&ast);
    let knows_owned = knows_owned::impl_knows_owned(&ast);
    quote! {
        #has_path_segment
        #has_branches
        #has_get
        #knows_owned
    }
}