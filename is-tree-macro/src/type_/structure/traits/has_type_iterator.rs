use proc_macro2::TokenStream;
use quote::quote;

use crate::{traits::AttributeQuery, type_::Structure};



pub fn impl_has_type_iterator(structure: &Structure) -> TokenStream {
    let has_type_iterator = structure.named_attribute_value(vec!["tree", "type_iterator"]);
    let knows_visitor = structure.named_attribute_value(vec!["tree", "relative_visitor"]);
    if let (Some(type_), Some(visitor)) = (has_type_iterator, knows_visitor) {
        let name = &structure.name;
        let consts: TokenStream = structure.fields.iter().map(|field| {
            let type_name = type_.get_ident().unwrap().to_string();
            let field_name = &field.field.ident;
            if let Some(_type_) = field.as_collection() {
                if field.is_any_type_of(&[type_.get_ident().unwrap().to_string().as_str()]) {
                    quote! { collection.extend(self.#field_name.iter().flat_map(|child| ::is_tree::Visitor::new(parent.clone(), child))); }
                } else {
                    quote! { collection.extend(self.#field_name.iter().flat_map(|child| child.iter_type_with_parent::<#type_>(::is_tree::Visitor::new(parent.clone().into(), self).into()))); }
                }
            } else {
                if field.is_any_type_of(&[type_name.as_str()]) {
                    quote! { collection.push(::is_tree::Visitor::new(parent.clone(), &self.#field_name)); }
                } else {
                    quote! { collection.extend((&self.#field_name).iter_type_with_parent::<#type_>(parent.clone())); }
                }
            }
        }).collect();

        let muts: TokenStream = structure.fields.iter().map(|field| {
            let type_name = type_.get_ident().unwrap().to_string();
            let field_name = &field.field.ident;
            if let Some(_type_) = field.as_collection() {
                if field.is_any_type_of(&[type_.get_ident().unwrap().to_string().as_str()]) {
                    quote! { collection.extend(self.#field_name.iter_mut().flat_map(|child| ::is_tree::Visitor::new(parent.clone(), child))); }
                } else {
                    quote! { collection.extend(self.#field_name.iter_mut().flat_map(|child| child.iter_type_with_parent::<#type_>(_self_as_parent.clone().into()))); }
                }
            } else {
                if field.is_any_type_of(&[type_name.as_str()]) {
                    quote! { collection.push(::is_tree::Visitor::new(parent.clone(), &mut self.#field_name)); }
                } else {
                    quote! { collection.extend((&mut self.#field_name).iter_type_with_parent::<#type_>(parent.clone())); }
                }
            }
        }).collect();
        quote! {
            impl<'a> ::is_tree::KnowsVisitorOf<'a, #type_> for &'a #name {
                type Visitor = ::is_tree::Visitor<#visitor, &'a #type_>;
            }

            impl<'a> ::is_tree::KnowsVisitorOf<'a, #type_> for &'a mut #name {
                type Visitor = ::is_tree::Visitor<#visitor, &'a mut #type_>;
            }
            
            impl<'a> ::is_tree::TypeIter<'a, ::is_tree::Visitor<#visitor, &'a #type_>> for &'a #name {
                fn type_iterator(self, parent: #visitor) -> ::is_tree::TypeIterator<::is_tree::Visitor<#visitor, &'a #type_>> {
                    use ::is_tree::{IterType, HasVisitor};
                    let mut collection = Vec::new();
                    #consts
                    collection.into()
                }
            }

            impl<'a> ::is_tree::TypeIter<'a, ::is_tree::Visitor<#visitor, &'a mut #type_>> for &'a mut #name {
                fn type_iterator(self, parent: #visitor) -> ::is_tree::TypeIterator<::is_tree::Visitor<#visitor, &'a mut #type_>> {
                    use ::is_tree::{IterType, HasVisitor};
                    let mut collection = Vec::new();
                    let self_ = unsafe { &mut *(self as *mut #name) };
                    let _self_as_parent: ::is_tree::Visitor<#visitor, &'a #name> = ::is_tree::Visitor::new(parent.clone().into(), unsafe { &mut *(self as *mut #name) }).into();
                    #muts
                    collection.into()
                }
            }
        }
    } else {
        quote! {}
    }
}