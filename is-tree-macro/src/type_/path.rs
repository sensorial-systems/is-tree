// TODO: Move it to its own module
#[derive(Clone)]
pub struct TypePath {
    pub path: syn::TypePath
}

impl TypePath {
    pub fn identifier(&self) -> &syn::Ident {
        &self.path.path.segments.last().unwrap().ident
    }

    pub fn inner_types(&self) -> Vec<Self> {
        self
            .path
            .path
            .segments
            .iter()
            .map(|segment| &segment.arguments)
            .map(|arguments| {
                if let syn::PathArguments::AngleBracketed(args) = arguments {
                    args
                        .args
                        .iter()
                        .filter_map(|arg| {
                            if let syn::GenericArgument::Type(ty) = arg {
                                Some(ty)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                }
            })
            .flatten()
            .filter_map(|ty| ty.clone().try_into().ok())
            .collect()
    }
}

impl TryFrom<syn::Type> for TypePath {
    type Error = ();
    fn try_from(ty: syn::Type) -> Result<Self, Self::Error> {
        match ty {
            syn::Type::Path(path) => Ok(Self { path }),
            syn::Type::Reference(reference) => (*reference.elem).try_into(),
            _ => Err(())
        }
    }
}
