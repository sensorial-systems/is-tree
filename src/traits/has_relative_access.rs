use crate::*;

pub trait KnowsRelativeAccessType<'a> {
    type RelativeType;
}

pub trait HasRelativeAccess<'a>: KnowsRelativeAccessType<'a> + KnowsPathSegment {
    fn relative<K>(&self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>;
}

impl<'a, T> KnowsRelativeAccessType<'a> for T
where Self: KnowsValue<'a>,
      <Self as KnowsValue<'a>>::Value: KnowsRelativeAccessType<'a>
{
    type RelativeType = <<Self as KnowsValue<'a>>::Value as KnowsRelativeAccessType<'a>>::RelativeType;
}

impl<'a, T> HasRelativeAccess<'a> for T
where
    Self: Into<Self::RelativeType> + Clone + HasValue<'a> + HasParent<'a> + KnowsRelativeAccessType<'a> + KnowsPathSegment,
    <Self as KnowsParent<'a>>::Parent: Into<Self::RelativeType>,

    Self: HasRoot<'a>,
    <Self as KnowsRoot<'a>>::Root:
        Into<Self::RelativeType>,

    Self: HasGet<'a>,
    <Self as KnowsGetType<'a>>::GetType:
        Into<Self::RelativeType>
        + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,

    Self::RelativeType:
        HasRelativeAccess<'a,
            RelativeType = <Self as KnowsRelativeAccessType<'a>>::RelativeType,
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
                // PathSegment::Other(_) => self.get(segment)?.into()
                _ => todo!()
            };
            visitor.relative(path)
        } else {
            Some(self.clone().into())
        }
    }
}
