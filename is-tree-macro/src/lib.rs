use proc_macro::TokenStream;
use quote::quote;

mod has_parent;
mod has_root;

#[proc_macro_derive(IsTree)]
pub fn is_tree(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let has_parent = has_parent::impl_has_parent(&ast);
    let has_root = has_root::impl_has_root(&ast);
    quote! {
        #has_parent
        #has_root
    }.into()
}
