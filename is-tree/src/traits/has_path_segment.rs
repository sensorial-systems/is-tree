use crate::{IsPathSegment, Path};

pub trait HasPath: KnowsPathSegment {
    fn path(&self) -> Path<Self::PathSegment>;
}

pub trait KnowsPathSegment {
    type PathSegment: IsPathSegment;
}

pub trait HasPathSegment: KnowsPathSegment {
    fn path_segment(&self) -> &Self::PathSegment;

    fn is(&self, identifier: impl PartialEq<Self::PathSegment>) -> bool {
        identifier.eq(self.path_segment())
    }

}

impl<T: KnowsPathSegment> KnowsPathSegment for &T {
    type PathSegment = T::PathSegment;
}

impl<T: HasPathSegment> HasPathSegment for &T {
    fn path_segment(&self) -> &Self::PathSegment {
        (*self).path_segment()
    }
}

impl<T: KnowsPathSegment> KnowsPathSegment for &mut T {
    type PathSegment = T::PathSegment;
}

impl<T: HasPathSegment> HasPathSegment for &mut T {
    fn path_segment(&self) -> &Self::PathSegment {
        (**self).path_segment()
    }
}
