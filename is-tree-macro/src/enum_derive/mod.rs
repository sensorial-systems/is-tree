mod has_parent;
mod has_root;
mod has_path_segment;
mod has_branches;
mod has_get;
mod has_relative_access;

use quote::quote;

pub fn impl_enum(ast: &syn::DeriveInput, data: &syn::DataEnum) -> proc_macro2::TokenStream {
    let has_path_segment = has_path_segment::impl_has_path_segment(&ast, data);
    let has_parent = has_parent::impl_has_parent(&ast, data);
    let has_root = has_root::impl_has_root(&ast, data);
    let has_get = has_get::impl_has_get(&ast, data);
    let has_branches = has_branches::impl_has_branches(&ast, data);
    let has_relative_access = has_relative_access::impl_has_relative_access(&ast, data);
    quote! {
        #has_path_segment
        #has_parent
        #has_root
        #has_branches
        #has_get
        #has_relative_access
    }
}