use crate::*;

impl<'a, Value> KnowsRelativeAccessType for RootVisitor<Value>
where Value: KnowsRelativeAccessType
{
    type RelativeType = Value::RelativeType;
}

impl<'a, Value> HasRelativeAccess for RootVisitor<Value>
where
    Self: Into<Self::RelativeType> + Clone,
    Value: KnowsPathSegment + KnowsRelativeAccessType,

    Self: HasGet,
    <Self as KnowsGetType>::GetType:
        Into<Self::RelativeType>
        + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,

    Self::RelativeType:
        HasRelativeAccess<
            RelativeType = <Self as KnowsRelativeAccessType>::RelativeType,
            PathSegment = <Self as KnowsPathSegment>::PathSegment
        >,
{
    fn relative<K>(&self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
        where K: Into<<Self as KnowsPathSegment>::PathSegment>,
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            match segment.kind() {
                PathSegment::Root | PathSegment::Self_ | PathSegment::Super => self.relative(path),
                PathSegment::Other(_segment) => 
                    self
                        .get(segment)
                        .and_then(|value| {
                            let visitor = value.into();
                            visitor.relative(path)
                        }),
            }
        } else {
            Some(self.clone().into())
        }
    }
}
