use is_tree::{KnowsPathSegment, HasPathSegment};

use super::Visitors;

impl<'a> KnowsPathSegment for &'a Visitors<'a> {
    type PathSegment = String;
}

impl<'a> HasPathSegment for &'a Visitors<'a> {
    fn path_segment(&self) -> &Self::PathSegment {
        match self {
            Visitors::Library(library) => library.path_segment(),
            Visitors::Module(module) => module.path_segment()
        }
    }
}

impl<'a> KnowsPathSegment for Visitors<'a> {
    type PathSegment = String;
}

impl<'a> HasPathSegment for Visitors<'a> {
    fn path_segment(&self) -> &Self::PathSegment {
        match self {
            Visitors::Library(visitor) => visitor.path_segment(),
            Visitors::Module(visitor) => visitor.path_segment()
        }
    }
}
