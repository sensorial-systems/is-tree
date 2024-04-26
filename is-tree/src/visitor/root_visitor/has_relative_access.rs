use crate::*;

impl<'a, Value> KnowsRelativeAccess<'a> for RootVisitor<Value>
where Value: KnowsRelativeAccess<'a>
{
    type RelativeAccess = Value::RelativeAccess;
}

impl<'a, Value> HasRelativeAccess<'a> for &'a RootVisitor<Value>
where
    RootVisitor<Value>: Into<Self::RelativeAccess> + Clone + KnowsRelativeAccess<'a> + 'a,
    Self: HasValue<'a> + HasParent<'a> + HasRoot<'a> + HasGet<'a>,
    <Self as KnowsParent<'a>>::Parent: Into<Self::RelativeAccess>,

    <Self as KnowsRoot<'a>>::Root:
        Into<Self::RelativeAccess>,

    <Self as KnowsBranches<'a>>::Branches:
        Into<Self::RelativeAccess> + HasPathSegment,

    Self::RelativeAccess:
        HasRelativeAccess<'a, RelativeAccess = <Self as KnowsRelativeAccess<'a>>::RelativeAccess>
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeAccess>
    where K: Into<String>
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
