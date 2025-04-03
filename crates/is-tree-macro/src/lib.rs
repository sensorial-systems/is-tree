use proc_macro::TokenStream;

mod traits;
mod type_;

use traits::Derive;
use type_::Type;

#[proc_macro_derive(IsTree, attributes(tree))]
pub fn is_tree(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    Type::from(ast).derive().into()
}
