use crate::{KnowsRelativeAccessType, Visitor, PathSegment, KnowsPathSegment, HasParent, HasRelativeAccess, KnowsParent, has_get::{KnowsGetType, HasGet}, KnowsRoot, HasRoot, IsPathSegment, KnowsVisitor};

impl<Parent, Value> KnowsRelativeAccessType for Visitor<Parent, Value>
where Value: KnowsRelativeAccessType
{
    type RelativeType = Value::RelativeType;
}

impl<'a, Parent, Value> KnowsRelativeAccessType for &'a Visitor<Parent, Value>
where Value: KnowsRelativeAccessType
{
    type RelativeType = Value::RelativeType;
}

impl<'a, Parent, Value> HasRelativeAccess for &'a Visitor<Parent, Value>
where
    Visitor<Parent, Value>: Into<Self::RelativeType> + Clone,
    Self: KnowsPathSegment,
    Parent: ToOwned + Clone,
    Parent::Owned: Into<Self::RelativeType>,
    Value: KnowsPathSegment + KnowsRelativeAccessType + KnowsVisitor,

    Self: HasRoot,
    <Self as KnowsRoot>::Root: ToOwned,
    <<Self as KnowsRoot>::Root as ToOwned>::Owned: Into<Self::RelativeType>,
    &'a Value::RelativeType: HasRoot<Root = <Self as KnowsRoot>::Root>,

    Self: HasGet,
    <Self as KnowsGetType>::GetType:
        KnowsVisitor
        + Into<Self::RelativeType>
        + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,
    Self: Into<<<<Self as KnowsGetType>::GetType as KnowsVisitor>::Visitor as KnowsParent>::Parent>,

    <Self as KnowsParent>::Parent: Into<Self::RelativeType>,
    &'a Value::RelativeType:
      HasRelativeAccess
    + KnowsRelativeAccessType<RelativeType = Self::RelativeType>
    + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>
    + HasParent,
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>
    {
            let mut path = path.into_iter();
            if let Some(segment) = path.next() {
                let segment = segment.into();
                let visitor = match segment.kind() {
                    PathSegment::Self_ => self.clone().into(),
                    PathSegment::Root => self.root().to_owned().into(),
                    PathSegment::Super => self.parent().to_owned().into(),
                    PathSegment::Other(_) => self.get(segment)?.into()
                };
                // FIXME: This is a hack.
                let visitor = unsafe { std::mem::transmute::<_, &'a Value::RelativeType>(&visitor) };
                visitor.relative(path)
            } else {
                Some(self.clone().into())
            }    
    }
}
