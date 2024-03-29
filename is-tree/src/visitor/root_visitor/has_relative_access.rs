use crate::*;

impl<'a, Value> KnowsRelativeAccessType<'a> for RootVisitor<Value>
where Value: KnowsRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}

impl<'a, Value> HasRelativeAccess<'a> for &'a RootVisitor<Value>
where
    RootVisitor<Value>: Into<Self::RelativeType> + Clone + KnowsRelativeAccessType<'a> + KnowsPathSegment + 'a,
    Self: HasValue<'a> + HasParent<'a> + HasRoot<'a> + HasGet<'a>,
    <Self as KnowsParent<'a>>::Parent: Into<Self::RelativeType>,

    <Self as KnowsRoot<'a>>::Root:
        Into<Self::RelativeType>,

    <Self as KnowsBranches<'a>>::Branches:
        Into<Self::RelativeType>
        + HasPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,

    Self::RelativeType:
        HasRelativeAccess<'a,
            RelativeType = <Self as KnowsRelativeAccessType<'a>>::RelativeType,
            PathSegment = <Self as KnowsPathSegment>::PathSegment
        >
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
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
