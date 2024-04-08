mod traits;
use traits::*;

mod variant;
use variant::*;

use quote::quote;

use crate::traits::{AttributeQuery, Derive};

pub struct Enumeration {
    generics: syn::Generics,
    attrs: Vec<syn::Attribute>,
    name: syn::Ident,
    variants: Vec<Variant>,
}

impl AttributeQuery for Enumeration {
    fn attributes(&self) -> &Vec<syn::Attribute> {
        &self.attrs
    }
}

impl From<(syn::DeriveInput, syn::DataEnum)> for Enumeration {
    fn from((ast, data): (syn::DeriveInput, syn::DataEnum)) -> Self {
        Self {
            attrs: ast.attrs,
            generics: ast.generics,
            name: ast.ident,
            variants: data.variants.into_iter().map(|field| field.into()).collect(),
        }
    }
}

impl Derive for Enumeration {
    fn derive(&self) -> proc_macro2::TokenStream {
        let has_path_segment = has_path_segment::impl_path_segment(self);
        let has_parent = has_parent::impl_has_parent(self);
        let has_root = has_root::impl_has_root(self);
        let has_get = has_get::impl_has_get(self);
        let has_branches = has_branches::impl_branches(self);
        let has_relative_access = has_relative_access::impl_relative_access(self);
        quote! {
            #has_path_segment
            #has_parent
            #has_root
            #has_branches
            #has_get
            #has_relative_access
        }
    }

}
