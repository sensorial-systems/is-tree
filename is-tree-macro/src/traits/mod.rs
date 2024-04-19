use quote::ToTokens;

pub trait AttributeQuery {
    fn attributes(&self) -> &Vec<syn::Attribute>;

    fn named_attribute_value(&self, path: Vec<&str>) -> Option<syn::Path> {
        self.attributes().iter().find_map(|attr| {
            if let (true, Ok(meta)) = (attr.path().is_ident(&path[0]), attr.parse_args::<syn::MetaNameValue>()) {
                if meta.path.is_ident(&path[1]) {
                    if let Some(value) = meta.value.to_token_stream().to_string().split('"').collect::<Vec<&str>>().get(1) {
                        if let Ok(syn::TypePath { path, .. }) = syn::parse_str(value) {
                            return Some(path);
                        }
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
    fn derive(&mut self) -> proc_macro2::TokenStream;
}