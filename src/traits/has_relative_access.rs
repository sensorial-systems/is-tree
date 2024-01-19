use crate::*;

pub trait KnowsRelativeAccessType {
    type RelativeType;
}

pub trait HasRelativeAccess: KnowsRelativeAccessType + KnowsPathSegment {
    fn relative<K>(&self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>;
}

impl<T> KnowsRelativeAccessType for T
where Self: KnowsValue,
      <Self as KnowsValue>::Value: KnowsRelativeAccessType
{
    type RelativeType = <<Self as KnowsValue>::Value as KnowsRelativeAccessType>::RelativeType;
}

impl<T> HasRelativeAccess for T
where
    Self: Into<Self::RelativeType> + Clone + HasValue + HasParent + KnowsRelativeAccessType + KnowsPathSegment,
    <Self as KnowsParent>::Parent: Into<Self::RelativeType>,

    Self: HasRoot,
    <Self as KnowsRoot>::Root:
        Into<Self::RelativeType>,

    Self: HasGet,
    <Self as KnowsGetType>::GetType:
        Into<Self::RelativeType>
        + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,

    Self::RelativeType:
        HasRelativeAccess<
            RelativeType = <Self as KnowsRelativeAccessType>::RelativeType,
            PathSegment = <Self as KnowsPathSegment>::PathSegment
        >
{
    fn relative<K>(&self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            let visitor = match segment.kind() {
                PathSegment::Self_ => self.clone().into(),
                PathSegment::Root => self.root().into(),
                PathSegment::Super => self.parent().into(),
                PathSegment::Other(_) => self.get(segment)?.into()
            };
            visitor.relative(path)
        } else {
            Some(self.clone().into())
        }    
    }
}
