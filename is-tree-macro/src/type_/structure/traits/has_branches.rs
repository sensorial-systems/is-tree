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

    // let mut consts = Vec::new();
    // let mut muts = Vec::new();

    // let mut branches = HashSet::new();
    
    let branches: HashSet<Path> = structure.fields.iter().map(|field| {
        field.attribute_group(vec!["tree", "branch"]).iter().cloned().map(Path::from).collect::<Vec<Path>>()
    }).flatten().collect();
    // for field in &structure.fields {
    //     if field.has_attribute(vec!["tree", "branch"]) {
    //         branches.insert(TokenStream::from(field.field.ty.to_token_stream()));
    //         let ident = field.field.ident.as_ref().expect("Unamed field not supported");
    //         if field.as_collection().is_some() {
    //             consts.push(quote! { self.#ident.iter().filter_map(|item| item.try_into().ok()) });
    //             muts.push(quote! { self.#ident.iter_mut().filter_map(|item| item.try_into().ok()) });
    //         } else {
    //             consts.push(quote! { std::iter::once(&self.#ident) });
    //             muts.push(quote! { std::iter::once(&mut self.#ident) });
    //         }
    //     }
    // }

    branches.iter().map(|branch| {
        let const_chain = structure.fields.iter().filter_map(|field| {
            let ident = &field.field.ident;
            if let Some(path) = field.as_collection() {
                if path.to_token_stream().to_string() == branch.path.to_token_stream().to_string() {
                    return Some(quote! { self.#ident.iter() })
                }
            } else if field.field.ty.to_token_stream().to_string() == branch.path.to_token_stream().to_string() {
                return Some(quote! { std::iter::once(&self.#ident) })
            }
            None
        }).collect::<proc_macro2::TokenStream>();

        let mut_chain = structure.fields.iter().filter_map(|field| {
            let ident = &field.field.ident;
            if let Some(path) = field.as_collection() {
                if path.to_token_stream().to_string() == branch.path.to_token_stream().to_string() {
                    return Some(quote! { self.#ident.iter_mut() })
                }
            } else if field.field.ty.to_token_stream().to_string() == branch.path.to_token_stream().to_string() {
                return Some(quote! { std::iter::once(&mut self.#ident) })
            }
            None
        }).collect::<proc_macro2::TokenStream>();

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

    // let mut const_chain = quote! {};
    // let mut mut_chain = quote! {};

    // for (index, (const_iter, mut_iter)) in consts.iter().zip(muts.iter()).enumerate() {
    //     if index == 0 {
    //         const_chain = quote! { #const_iter };
    //         mut_chain = quote! { #mut_iter };
    //     } else {
    //         const_chain = quote! { #const_chain.chain(#const_iter) };
    //         mut_chain = quote! { #mut_chain.chain(#mut_iter) };
    //     }
    // }

    // if const_chain.is_empty() || mut_chain.is_empty() {
    //     quote! {}
    // } else {
    //     structure.implementation.has_branches = true;
    //     quote! {
    //         impl<'a> ::is_tree::HasBranches<&'a Module> for &'a #structure_name {
    //             fn branches_impl(self) -> impl Iterator<Item = &'a Module> {
    //                 #const_chain
    //             }
    //         } 
    
    //         impl<'a> ::is_tree::HasBranches<&'a mut Module> for &'a mut #structure_name {
    //             fn branches_impl(self) -> impl Iterator<Item = &'a mut Module> {
    //                 #mut_chain
    //             }
    //         }
    //     }
    // }
}

pub(crate) fn impl_branches(structure: &mut Structure) -> proc_macro2::TokenStream {
    let has = impl_has_branches(structure);
    quote! {
        #has
    }
}