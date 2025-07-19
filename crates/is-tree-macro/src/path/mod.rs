use std::hash::{Hash, Hasher};

use quote::ToTokens;

/// Wrapper for syn::Path to enable hashing and equality based on token stream.
pub struct Path {
    pub path: syn::Path
}

impl Hash for Path {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.to_token_stream().to_string().hash(state);
    }
}

impl From<syn::Path> for Path {
    fn from(path: syn::Path) -> Self {
        Self { path }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.path.to_token_stream().to_string() == other.path.to_token_stream().to_string()
    }
}

impl Eq for Path {}
