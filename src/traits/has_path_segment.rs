use crate::{IsPathSegment, Path};

pub trait HasPath<PathSegment> {
    fn path(&self) -> Path<PathSegment>;
}

impl<PathSegment> HasPath<PathSegment> for () {
    fn path(&self) -> Path<PathSegment> {
        Default::default()
    }
}

pub trait KnowsPathSegment {
    type PathSegment: IsPathSegment;
}

// TODO: Remove the need of this:
impl<T: KnowsPathSegment> KnowsPathSegment for &T {
    type PathSegment = T::PathSegment;
}

// TODO: Remove the need of this:
impl<T: HasPathSegment> HasPathSegment for &T {
    fn path_segment(&self) -> &Self::PathSegment {
        (*self).path_segment()
    }
}

pub trait HasPathSegment: KnowsPathSegment {
    fn path_segment(&self) -> &Self::PathSegment;

    fn is(&self, identifier: impl PartialEq<Self::PathSegment>) -> bool {
        identifier.eq(self.path_segment())
    }

}

impl KnowsPathSegment for String {
    type PathSegment = Self;
}

impl HasPathSegment for String {
    fn path_segment(&self) -> &Self::PathSegment {
        self
    }
}

impl KnowsPathSegment for () {
    type PathSegment = ();
}

impl HasPathSegment for () {
    fn path_segment(&self) -> &Self::PathSegment {
        self
    }
}
