use crate::type_::TypePath;

pub struct Variant {
    pub variant: syn::Variant,
    pub fields: Vec<TypePath>
}

impl Variant {
    pub fn fields(&self) -> &Vec<TypePath> {
        &self.fields
    }
}

impl From<syn::Variant> for Variant {
    fn from(variant: syn::Variant) -> Self {
        let fields = variant
            .fields
            .iter()
            .cloned()
            .filter_map(|field| 
                field
                    .ty
                    .try_into()
                    .ok()
            ).collect();
        Self { variant, fields }
    }
}
