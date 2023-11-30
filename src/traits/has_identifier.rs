use crate::IsPathSegment;

pub trait HasPathSegment {
    type PathSegment: IsPathSegment;
    fn path_segment(&self) -> &Self::PathSegment;
}

impl HasPathSegment for String {
    type PathSegment = Self;
    fn path_segment(&self) -> &Self::PathSegment {
        self
    }
}