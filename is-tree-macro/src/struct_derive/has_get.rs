use quote::quote;

pub(crate) fn impl_has_get(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structure_name = &ast.ident;
    quote! {
        impl<'a> ::is_tree::HasGet<'a> for &'a #structure_name {}
        impl<'a> ::is_tree::HasGet<'a> for &'a mut #structure_name {}

        impl<'a> ::is_tree::HasGetOrCreate<'a> for #structure_name {}
    }
}