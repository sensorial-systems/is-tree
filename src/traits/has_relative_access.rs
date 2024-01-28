use crate::*;

pub trait KnowsRelativeAccessType<'a> {
    type RelativeType;
}

pub trait HasRelativeAccess<'a>: KnowsRelativeAccessType<'a> + KnowsPathSegment {
    fn relative<K>(&self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>;
}

impl<'a, T: KnowsRelativeAccessType<'a>> KnowsRelativeAccessType<'a> for &'a T {
    type RelativeType = T::RelativeType;
}