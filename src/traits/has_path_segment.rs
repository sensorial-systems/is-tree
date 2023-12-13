use crate::{IsPathSegment, Path};

pub trait HasPath<PathSegment> {
    fn path(&self) -> Path<PathSegment>;
}

impl<PathSegment> HasPath<PathSegment> for () {
    fn path(&self) -> Path<PathSegment> {
        Default::default()
    }
}

pub trait HasPathSegment {
    type PathSegment: IsPathSegment;

    fn path_segment(&self) -> &Self::PathSegment;

    fn is(&self, identifier: impl PartialEq<Self::PathSegment>) -> bool {
        identifier.eq(self.path_segment())
    }

}

impl HasPathSegment for String {
    type PathSegment = Self;
    fn path_segment(&self) -> &Self::PathSegment {
        self
    }
}

impl HasPathSegment for () {
    type PathSegment = ();
    fn path_segment(&self) -> &Self::PathSegment {
        self
    }
}

impl HasPathSegment for &() {
    type PathSegment = ();
    fn path_segment(&self) -> &Self::PathSegment {
        self
    }
}
