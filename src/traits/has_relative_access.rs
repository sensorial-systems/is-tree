use crate::HasPathSegment;

pub trait KnowsRelativeAccessType<'a> {
    type RelativeType;
}

pub trait HasRelativeAccess<'a>: KnowsRelativeAccessType<'a> + HasPathSegment {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as HasPathSegment>::PathSegment>;
}
