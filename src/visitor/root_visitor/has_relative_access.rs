use crate::{KnowsRelativeAccessType, RootVisitor, HasRelativeAccess, KnowsPathSegment, has_get::{HasGet, KnowsGetType}, HasRoot, PathSegment, IsPathSegment, KnowsVisitor, KnowsParent};

impl<'a, Value> KnowsRelativeAccessType for &'a RootVisitor<Value>
where Value: KnowsRelativeAccessType
{
    type RelativeType = Value::RelativeType;
}

impl<'a, Value> HasRelativeAccess for &'a RootVisitor<Value>
where
    Value: Copy + KnowsPathSegment + HasGet,
    Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsVisitor,
    RootVisitor<Value>: Into<<<Value::GetType as KnowsVisitor>::Visitor as KnowsParent>::Parent>,
    Value: KnowsRelativeAccessType + KnowsPathSegment + Clone + Copy + 'a,
    Self: HasRoot,
    RootVisitor<Value>: Into<Self::RelativeType>,

    Self: HasGet,
    <Self as KnowsGetType>::GetType:
        KnowsVisitor
        + Into<Self::RelativeType>
        + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,
    RootVisitor<Value>: Into<<<<Self as KnowsGetType>::GetType as KnowsVisitor>::Visitor as KnowsParent>::Parent>,

    &'a Self::RelativeType:
        HasRelativeAccess<
            RelativeType = <Self as KnowsRelativeAccessType>::RelativeType,
            PathSegment = <Self as KnowsPathSegment>::PathSegment
        >
        + HasRoot,
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
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
                            // FIXME: This is a hack.
                            let visitor = value.into();
                            let visitor = unsafe { std::mem::transmute::<_, &'a Self::RelativeType>(&visitor) };
                            visitor.relative(path)
                        }),
            }
        } else {
            Some(self.clone().into())
        }
    }
}
