use is_tree::{KnowsPathSegment, HasPathSegment};

use super::Library;

impl KnowsPathSegment for Library {
    type PathSegment = String;
}

impl HasPathSegment for Library {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}
