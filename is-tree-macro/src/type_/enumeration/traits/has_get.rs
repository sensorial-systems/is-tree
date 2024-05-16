use proc_macro2::TokenStream;
use quote::quote;

use crate::type_::Enumeration;

pub fn impl_has_get(enumeration: &Enumeration) -> TokenStream {
    let name = &enumeration.name;
    let generics = &enumeration.generics;
    let self_ = quote! { #name #generics };
    let generics = enumeration.generics_with_lifetime();
    quote! {
        impl #generics ::is_tree::HasGet<'a> for #self_
        where #self_: ::is_tree::HasBranches<'a>,
                <#self_ as ::is_tree::KnowsBranches<'a>>::Branches: ::is_tree::HasPathSegment
        {}
    }
}