use crate::traits::AttributeQuery;

pub struct Field {
    pub field: syn::Field
}

impl Field {
    pub fn is_collection(&self) -> bool {
        self.is_any_type_of(&["Vec", "Option"])
    }

    pub fn is_any_type_of(&self, types: &[&str]) -> bool {
        match &self.field.ty {
            syn::Type::Path(syn::TypePath { path, .. }) => {
                path.segments.first().map(|path| types.contains(&&*path.ident.to_string())).unwrap_or_default()
            },
            _ => false
        }
    }
}

impl AttributeQuery for Field {
    fn attributes(&self) -> &Vec<syn::Attribute> {
        &self.field.attrs
    }
}

impl From<syn::Field> for Field {
    fn from(field: syn::Field) -> Self {
        Self {
            field
        }
    }
}
