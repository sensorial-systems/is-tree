use std::collections::HashSet;

use quote::{quote, ToTokens};

use crate::{path::Path, traits::AttributeQuery, type_::Structure};

/// Collects unique branch types from the structure's fields based on attributes.
fn collect_branch_types(structure: &Structure) -> HashSet<Path> {
    let all_branches = structure.has_attribute(vec!["tree", "branches"]);
    structure.fields.iter().filter_map(|field| {
        if all_branches || field.has_attribute(vec!["tree", "branch"]) {
            if let Some(path) = field.as_collection() {
                Some(path.clone().into())
            } else if let syn::Type::Path(type_path) = &field.field.ty {
                Some(Path::from(type_path.path.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }).collect()
}

/// Gets fields that contribute to a specific branch type.
fn get_contributing_fields<'a>(structure: &'a Structure, branch: &'a Path, all_branches: bool) -> Vec<&'a super::super::field::Field> {
    structure.fields.iter().filter(|field| {
        if all_branches || field.has_attribute(vec!["tree", "branch"]) {
            if let Some(field_type) = field.as_collection() {
                field_type.to_token_stream().to_string() == branch.path.to_token_stream().to_string()
            } else if let syn::Type::Path(type_path) = &field.field.ty {
                type_path.path.to_token_stream().to_string() == branch.path.to_token_stream().to_string()
            } else {
                false
            }
        } else {
            false
        }
    }).collect()
}

/// Generates the constant iterator expression for a field.
fn generate_const_iterator(field: &super::super::field::Field) -> proc_macro2::TokenStream {
    let field_name = field.field.ident.as_ref().unwrap();
    if field.as_collection().is_some() {
        quote! { self.#field_name.iter() }
    } else {
        quote! { std::iter::once(&self.#field_name) }
    }
}

/// Generates the mutable iterator expression for a field.
fn generate_mut_iterator(field: &super::super::field::Field) -> proc_macro2::TokenStream {
    let field_name = field.field.ident.as_ref().unwrap();
    if field.as_collection().is_some() {
        quote! { self.#field_name.iter_mut() }
    } else {
        quote! { std::iter::once(&mut self.#field_name) }
    }
}

/// Implements the HasBranches trait for the structure.
pub(crate) fn impl_has_branches(structure: &mut Structure) -> proc_macro2::TokenStream {
    let structure_name = &structure.name;
    let all_branches = structure.has_attribute(vec!["tree", "branches"]);
    let branches = collect_branch_types(structure);
    branches.iter().map(|branch| {
        let contributing_fields = get_contributing_fields(structure, branch, all_branches);
        let const_iters: Vec<_> = contributing_fields.iter().map(|&field| generate_const_iterator(field)).collect();
        let mut_iters: Vec<_> = contributing_fields.iter().map(|&field| generate_mut_iterator(field)).collect();
        let branch = &branch.path;
        quote! {
            impl<'a> ::is_tree::HasBranches<&'a #branch> for &'a #structure_name {
                fn branches_impl(self) -> impl Iterator<Item = &'a #branch> {
                    std::iter::empty() #(.chain(#const_iters))*
                }
            }
            impl<'a> ::is_tree::HasBranches<&'a mut #branch> for &'a mut #structure_name {
                fn branches_impl(self) -> impl Iterator<Item = &'a mut #branch> {
                    std::iter::empty() #(.chain(#mut_iters))*
                }
            }
        }
    }).collect()
}

/// Wraps the impl_has_branches to generate the branches implementation.
pub(crate) fn impl_branches(structure: &mut Structure) -> proc_macro2::TokenStream {
    let has = impl_has_branches(structure);
    quote! {
        #has
    }
}