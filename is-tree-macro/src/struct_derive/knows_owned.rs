use quote::quote;



pub(crate) fn impl_knows_owned(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structure_name = &ast.ident;
    quote! {
        impl ::is_tree::KnowsOwned for #structure_name {
            type Owned = #structure_name;
        }
    }
}