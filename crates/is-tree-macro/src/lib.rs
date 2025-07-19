use proc_macro::TokenStream;

mod traits;
mod type_;
mod path;

use traits::Derive;
use type_::Type;

#[proc_macro_derive(IsTree, attributes(tree))]
pub fn is_tree(input: TokenStream) -> TokenStream {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("proc-macro panic: {}", info);
    }));
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);
    Type::from(ast).derive().into()
}
