use quote::quote;

pub(crate) fn impl_knows_branches(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let structure_name = &ast.ident;
    let branches = ast.attrs.iter().find_map(|attr| {
        if let (true, Ok(meta)) = (attr.path().is_ident("tree"), attr.parse_args::<syn::MetaNameValue>()) {
            if meta.path.is_ident("branches") {
                match meta.value {
                    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(value), .. }) => {
                        Some(syn::Ident::new(&value.value(), value.span()))
                    },
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
        }
    }).unwrap_or(structure_name.clone());
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

pub(crate) fn impl_has_branches(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let structure_name = &ast.ident;

    let mut const_chain = quote! {};
    let mut mut_chain = quote! {};

    if let syn::Fields::Named(fields) = &data.fields {
        for field in &fields.named {
            if field.attrs.iter().any(|attr| attr.path().is_ident("tree") && attr.parse_args::<syn::Path>().map(|path| path.is_ident("branch")).unwrap_or_default()) {
                match &field.ty {
                    // Check if type is a Vec
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        let is_vec = path
                            .segments
                            .first()
                            .map(|path| path.ident == "Vec")
                            .unwrap_or_default();
                        if is_vec {
                            let ident = field.ident.as_ref().expect("Unamed field not supported");
                            const_chain = quote! {
                                #const_chain
                                self.#ident.iter().filter_map(|item| item.try_into().ok())
                            };
                            mut_chain = quote! {
                                #mut_chain
                                self.#ident.iter_mut().filter_map(|item| item.try_into().ok())
                            };
                        } else {
                            let ident = field.ident.as_ref().expect("Unamed field not supported");
                            const_chain = quote! {
                                #const_chain
                                std::iter::once(&self.#ident)
                            };
                            mut_chain = quote! {
                                #mut_chain
                                std::iter::once(&mut self.#ident)
                            };
                        }
                    },
                    _ => ()
                };
            }
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

pub(crate) fn impl_branches(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let knows = impl_knows_branches(ast);
    let has = impl_has_branches(ast, data);
    quote! {
        #knows
        #has
    }
}