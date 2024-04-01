use proc_macro::TokenStream;

mod enum_derive;
mod struct_derive;

#[proc_macro_derive(IsTree, attributes(tree))]
pub fn is_tree(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    match ast.data {
        syn::Data::Enum(_) => enum_derive::impl_enum(&ast),
        syn::Data::Struct(_) => struct_derive::impl_struct(&ast),
        syn::Data::Union(_) => panic!("IsTree cannot be derived for unions"),
    }.into()
}
