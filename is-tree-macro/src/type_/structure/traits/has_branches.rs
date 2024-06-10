// TODO: Clean this up.

use std::{collections::HashSet, hash::{Hash, Hasher}};

use quote::{quote, ToTokens};

use crate::{traits::AttributeQuery, type_::Structure};

pub struct Path {
    pub path: syn::Path
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.to_token_stream().to_string().hash(state);
    }
}

impl From<syn::Path> for Path {
    fn from(path: syn::Path) -> Self {
        Self { path }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.path.to_token_stream().to_string() == other.path.to_token_stream().to_string()
    }
}

impl Eq for Path {}

pub(crate) fn impl_has_branches(structure: &mut Structure) -> proc_macro2::TokenStream {
    let structure_name = &structure.name;

    let all_branches = structure.has_attribute(vec!["tree", "branches"]);

    let branches: HashSet<Path> = structure.fields.iter().filter_map(|field| {
        if all_branches || field.has_attribute(vec!["tree", "branch"]) {
            if let Some(path) = field.as_collection() {
                Some(path.clone().into())
            } else if let syn::Type::Path(path) = &field.field.ty {
                Some(Path::from(path.path.clone()))
            } else {
                None
            }
        } else {
            None
        }
    }).collect();

    branches.iter().map(|branch| { // e.g. Module
        let mut const_chain = structure.fields.iter().filter_map(|field| {
            let ident = &field.field.ident;
            if let Some(path) = field.as_collection() { // e.g. Vec<Module>
                let path = Path::from(path.clone());
                if (all_branches || field.has_attribute(vec!["tree", "branch"])) && branch.path.to_token_stream().to_string() == path.path.to_token_stream().to_string() { // e.g. #[tree(branch(Module))]
                    if path.path.to_token_stream().to_string() == branch.path.to_token_stream().to_string() { // e.g. Branch == Module and Vec<Module>
                        return Some(quote! { self.#ident.iter() })
                    } else { // e.g. Branch != Module and Vec<Module>
                        let branch = &branch.path;
                        return Some(quote! { self.#ident.iter().flat_map(|branch| branch.branches_impl2::<&#branch>()).collect::<Vec<_>>() })
                    }
                }
            } else {
                let type_ = &field.field.ty;
                let type_path: syn::Path = syn::parse_quote! { #type_ };
                let path = Path::from(type_path);
                if (all_branches || field.has_attribute(vec!["tree", "branch"])) && branch.path.to_token_stream().to_string() == path.path.to_token_stream().to_string() { // e.g. Is not Vec<Module> and #[tree(branch(Module))]
                    if field.field.ty.to_token_stream().to_string() == branch.path.to_token_stream().to_string() { // e.g. Branch == Module
                        return Some(quote! { std::iter::once(&self.#ident) })
                    } else { // e.g. Branch != Module
                        let branch = &branch.path;
                        return Some(quote! { (&self.#ident).branches_impl2::<&#branch>() })
                    }
                }
            }
            None
        });
        let const_chain_first = const_chain.next().unwrap_or_default();
        let const_chain = const_chain.fold(const_chain_first, |acc, iter| quote! { #acc.chain(#iter) });

        let mut mut_chain = structure.fields.iter().filter_map(|field| {
            let ident = &field.field.ident;
            if let Some(path) = field.as_collection() { // e.g. Vec<Module>
                let path = Path::from(path.clone());
                if (all_branches || field.has_attribute(vec!["tree", "branch"])) && branch.path.to_token_stream().to_string() == path.path.to_token_stream().to_string() { // e.g. #[tree(branch(Module))]
                    if path.path.to_token_stream().to_string() == branch.path.to_token_stream().to_string() { // e.g. Branch == Module and Vec<Module>
                        return Some(quote! { self.#ident.iter_mut() })
                    } else { // e.g. Branch != Module and Vec<Module>
                        let branch = &branch.path;
                        return Some(quote! { self.#ident.iter_mut().flat_map(|branch| branch.branches_impl2::<&mut #branch>()).collect::<Vec<_>>() })
                    }
                }
            } else {
                let type_ = &field.field.ty;
                let type_path: syn::Path = syn::parse_quote! { #type_ };
                let path = Path::from(type_path);
                if (all_branches || field.has_attribute(vec!["tree", "branch"])) && branch.path.to_token_stream().to_string() == path.path.to_token_stream().to_string() { // e.g. Is not Vec<Module> and #[tree(branch(Module))]
                    if field.field.ty.to_token_stream().to_string() == branch.path.to_token_stream().to_string() { // e.g. Branch == Module
                        return Some(quote! { std::iter::once(&mut self.#ident) })
                    } else { // e.g. Branch != Module
                        let branch = &branch.path;
                        return Some(quote! { (&mut self.#ident).branches_impl2::<&mut #branch>() })
                    }
                }
            }
            None
        });
        let mut_chain_first = mut_chain.next().unwrap_or_default();
        let mut_chain = mut_chain.fold(mut_chain_first, |acc, iter| quote! { #acc.chain(#iter) });

        let branch = &branch.path;
        quote! {
            impl<'a> ::is_tree::HasBranches<&'a #branch> for &'a #structure_name {
                fn branches_impl(self) -> impl Iterator<Item = &'a #branch> {
                    #const_chain
                }
            } 
            
            impl<'a> ::is_tree::HasBranches<&'a mut #branch> for &'a mut #structure_name {
                fn branches_impl(self) -> impl Iterator<Item = &'a mut #branch> {
                    #mut_chain
                }
            }
        }
    }).collect()
}

pub(crate) fn impl_branches(structure: &mut Structure) -> proc_macro2::TokenStream {
    let has = impl_has_branches(structure);
    quote! {
        #has
    }
}