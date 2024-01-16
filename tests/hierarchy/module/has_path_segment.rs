use is_tree::{KnowsPathSegment, HasPathSegment};

use super::Module;

impl KnowsPathSegment for Module {
    type PathSegment = String;
}

impl HasPathSegment for Module {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl KnowsPathSegment for &Module {
    type PathSegment = String;
}

impl HasPathSegment for &Module {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}
