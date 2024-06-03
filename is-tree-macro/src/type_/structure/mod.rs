use quote::quote;

mod traits;
use traits::*;

mod field;
use field::*;

use crate::traits::{Derive, AttributeQuery};

#[derive(Default)]
pub struct Implementation {
    pub has_branches: bool,
    pub has_path_segment: bool
}

pub struct Structure {
    pub attrs: Vec<syn::Attribute>,
    pub name: syn::Ident,
    pub fields: Vec<Field>,
    pub implementation: Implementation
}

impl From<(syn::DeriveInput, syn::DataStruct)> for Structure {
    fn from((ast, data): (syn::DeriveInput, syn::DataStruct)) -> Self {
        let implementation = Default::default();
        Self {
            attrs: ast.attrs,
            name: ast.ident,
            fields: data.fields.into_iter().map(|field| field.into()).collect(),
            implementation
        }
    }
}

impl AttributeQuery for Structure {
    fn attributes(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl Derive for Structure {
    fn derive(&mut self) -> proc_macro2::TokenStream {
        let has_path_segment = has_path_segment::impl_has_path_segment(self);
        let has_branches = has_branches::impl_branches(self);
        // let has_get = has_get::impl_has_get(self);
        // let knows_owned = knows_owned::impl_knows_owned(self);
        // let has_type_iterator = has_type_iterator::impl_has_type_iterator(self);
        // let knows_relative_type = knows_relative_type::impl_knows_relative_type(self);
        // let knows_visitor = knows_visitor::impl_knows_visitor(self);
        quote! {
            #has_path_segment
            #has_branches
            // #has_get
            // #knows_owned
            // #has_type_iterator
            // #knows_relative_type
            // #knows_visitor
        }
    }
}
