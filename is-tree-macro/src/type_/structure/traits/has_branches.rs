use quote::quote;

use crate::{traits::AttributeQuery, type_::Structure};


pub(crate) fn impl_knows_branches(structure: &Structure) -> proc_macro2::TokenStream {
    let structure_name = &structure.name;
    let branches = structure
        .named_attribute_value(vec!["tree", "branches"])
        .unwrap_or_else(|| structure_name.clone());
    quote! {
        impl<'a> ::is_tree::KnowsBranches<'a> for #structure_name {
            type Branches = #branches;
        }

        impl<'a> ::is_tree::KnowsBranches<'a> for &'a #structure_name {
            type Branches = &'a #branches;
        }

        impl<'a> ::is_tree::KnowsBranches<'a> for &'a mut #structure_name {
            type Branches = &'a mut #branches;
        }
    }
}

pub(crate) fn impl_has_branches(structure: &Structure) -> proc_macro2::TokenStream {
    let structure_name = &structure.name;

    let mut consts = Vec::new();
    let mut muts = Vec::new();

    for field in &structure.fields {
        if field.has_attribute(vec!["tree", "branch"]) {
            let ident = field.field.ident.as_ref().expect("Unamed field not supported");
            if field.is_collection() {
                consts.push(quote! { self.#ident.iter().filter_map(|item| item.try_into().ok()) });
                muts.push(quote! { self.#ident.iter_mut().filter_map(|item| item.try_into().ok()) });
            } else {
                consts.push(quote! { std::iter::once(&self.#ident) });
                muts.push(quote! { std::iter::once(&mut self.#ident) });
            }
        }
    }

    let mut const_chain = quote! {};
    let mut mut_chain = quote! {};

    for (index, (const_iter, mut_iter)) in consts.iter().zip(muts.iter()).enumerate() {
        if index == 0 {
            const_chain = quote! { #const_iter };
            mut_chain = quote! { #mut_iter };
        } else {
            const_chain = quote! { #const_chain.chain(#const_iter) };
            mut_chain = quote! { #mut_chain.chain(#mut_iter) };
        }
    }

    if const_chain.is_empty() || mut_chain.is_empty() {
        quote! {}
    } else {
        quote! {
            impl<'a> ::is_tree::HasBranches<'a> for &'a #structure_name {
                fn branches(self) -> impl Iterator<Item = Self::Branches> {
                    #const_chain
                }
            } 
    
            impl<'a> ::is_tree::HasBranches<'a> for &'a mut #structure_name {
                fn branches(self) -> impl Iterator<Item = Self::Branches> {
                    #mut_chain
                }
            }
        }
    }
}

pub(crate) fn impl_branches(structure: &Structure) -> proc_macro2::TokenStream {
    let knows = impl_knows_branches(structure);
    let has = impl_has_branches(structure);
    quote! {
        #knows
        #has
    }
}