pub trait AttributeQuery {
    fn attributes(&self) -> &Vec<syn::Attribute>;

    fn named_attribute_value(&self, path: Vec<&str>) -> Option<syn::Ident> {
        self.attributes().iter().find_map(|attr| {
            if let (true, Ok(meta)) = (attr.path().is_ident(&path[0]), attr.parse_args::<syn::MetaNameValue>()) {
                if meta.path.is_ident(&path[1]) {
                    if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(value), .. }) = meta.value {
                        return Some(syn::Ident::new(&value.value(), value.span()))
                    }
                }
            }
            None
        })
    }

    fn has_attribute(&self, path: Vec<&str>) -> bool {
        self
            .attributes()
            .iter()
            .any(|attr|
                    attr.path().is_ident(&path[0]) &&
                    attr
                        .parse_args::<syn::Path>().map(|other_path| other_path.is_ident(&path[1]))
                        .unwrap_or_default())
    }
}

pub trait Derive {
    fn derive(&self) -> proc_macro2::TokenStream;
}