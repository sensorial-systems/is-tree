use crate::KnowsPathSegment;

pub trait KnowsRelativeAccessType {
    type RelativeType;
}

pub trait HasRelativeAccess: KnowsRelativeAccessType + KnowsPathSegment {
    fn relative<K>(&self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>;
}
