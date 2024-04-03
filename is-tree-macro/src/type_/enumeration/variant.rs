pub struct Variant {
    pub variant: syn::Variant
}

impl From<syn::Variant> for Variant {
    fn from(variant: syn::Variant) -> Self {
        Self {
            variant
        }
    }
}
