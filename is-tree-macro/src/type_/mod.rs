use crate::traits::Derive;

mod structure;

pub use structure::*;

pub enum Type {
    Structure(Structure)
}

impl Derive for Type {
    fn derive(&mut self) -> proc_macro2::TokenStream {
        match self {
            Type::Structure(structure) => structure.derive(),
        }
    }
}

impl From<syn::DeriveInput> for Type {
    fn from(ast: syn::DeriveInput) -> Self {
        match ast.data.clone() {
            syn::Data::Struct(data) => Type::Structure(Structure::from((ast, data))),
            _ => panic!("IsTree cannot be derived for unions and enumerations"),
        }
    }
}
