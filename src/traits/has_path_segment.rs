use crate::IsPathSegment;

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
