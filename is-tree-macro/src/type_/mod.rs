use crate::traits::Derive;

mod enumeration;
mod structure;

pub use enumeration::*;
pub use structure::*;

pub enum Type {
    Enumeration(Enumeration),
    Structure(Structure)
}

impl Derive for Type {
    fn derive(&self) -> proc_macro2::TokenStream {
        match self {
            Type::Enumeration(enumeration) => enumeration.derive(),
            Type::Structure(structure) => structure.derive(),
        }
    }
}

impl From<syn::DeriveInput> for Type {
    fn from(ast: syn::DeriveInput) -> Self {
        match ast.data.clone() {
            syn::Data::Enum(data) => Type::Enumeration(Enumeration::from((ast, data))),
            syn::Data::Struct(data) => Type::Structure(Structure::from((ast, data))),
            syn::Data::Union(_) => panic!("IsTree cannot be derived for unions"),
        }
    }
}
