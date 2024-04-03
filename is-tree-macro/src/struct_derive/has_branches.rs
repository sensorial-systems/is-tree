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

    let mut consts = Vec::new();
    let mut muts = Vec::new();

    if let syn::Fields::Named(fields) = &data.fields {
        for field in &fields.named {
            if field.attrs.iter().any(|attr| attr.path().is_ident("tree") && attr.parse_args::<syn::Path>().map(|path| path.is_ident("branch")).unwrap_or_default()) {
                match &field.ty {
                    syn::Type::Path(syn::TypePath { path, .. }) => {
                        let is_collection = path
                            .segments
                            .first()
                            .map(|path| path.ident == "Vec" || path.ident == "Option")
                            .unwrap_or_default();
                        let ident = field.ident.as_ref().expect("Unamed field not supported");
                        if is_collection {
                            consts.push(quote! { self.#ident.iter().filter_map(|item| item.try_into().ok()) });
                            muts.push(quote! { self.#ident.iter_mut().filter_map(|item| item.try_into().ok()) });
                        } else {
                            consts.push(quote! { std::iter::once(&self.#ident) });
                            muts.push(quote! { std::iter::once(&mut self.#ident) });
                        }
                    },
                    _ => ()
                };
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

pub(crate) fn impl_branches(ast: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let knows = impl_knows_branches(ast);
    let has = impl_has_branches(ast, data);
    quote! {
        #knows
        #has
    }
}