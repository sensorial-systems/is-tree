use quote::quote;



pub(crate) fn impl_has_branches(ast: &&syn::DeriveInput) -> proc_macro2::TokenStream {
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