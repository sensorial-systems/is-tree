use crate::HasPathSegment;

pub trait HasRelativeAccessType<'a> {
    type RelativeType;
}

pub trait HasRelativeAccess<'a>: HasRelativeAccessType<'a> + HasPathSegment {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as HasPathSegment>::PathSegment>;
}
