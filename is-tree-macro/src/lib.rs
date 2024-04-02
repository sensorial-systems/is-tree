use proc_macro::TokenStream;

mod enum_derive;
mod struct_derive;

#[proc_macro_derive(IsTree, attributes(tree))]
pub fn is_tree(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    match &ast.data {
        syn::Data::Enum(data) => enum_derive::impl_enum(&ast, data),
        syn::Data::Struct(data) => struct_derive::impl_struct(&ast, data),
        syn::Data::Union(_) => panic!("IsTree cannot be derived for unions"),
    }.into()
}
