use super::*;

#[derive(Clone, Copy, Default)]
pub struct RootVisitor<Value>
where Value: HasPathSegment
{
    pub value: Value
}

impl<'a, Value> KnowsGetType<'a> for RootVisitor<Value>
where Value: HasPathSegment + KnowsGetType<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Value> KnowsParentVisitor<'a> for RootVisitor<Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>
{
    type ParentVisitor = Value::ParentVisitor;
}

impl<'a, Value> IsVisitor<'a, Value> for RootVisitor<Value>
where Value: HasPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: HasPathSegment<PathSegment = Value::PathSegment>,
          Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}

impl<'a, Value> HasGet<'a> for RootVisitor<Value>
where Value: Copy + HasPathSegment + HasGet<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as HasPathSegment>::PathSegment> {
        self.value.get(key).map(|value| self.visit(value))
    }
}

impl<'a, Value> HasRoot<'a> for RootVisitor<Value>
where Value: HasPathSegment
{
    type Root = Self;
    fn root(self) -> Self {
        self
    }
}

impl<'a, Value> HasRoot<'a> for &'a RootVisitor<Value>
where Value: HasPathSegment
{
    type Root = Self;
    fn root(self) -> Self {
        self
    }
}

impl<'a, Value> KnowsParent<'a> for RootVisitor<Value>
where Value: HasPathSegment
{
    type Parent = Self;
}

impl<'a, Value> HasParent<'a> for RootVisitor<Value>
where Value: HasPathSegment
{
    fn parent(self) -> Self {
        self
    }
}

impl<'a, Value> RootVisitor<Value>
where Value: HasPathSegment + KnowsRelativeAccessType<'a>, Value::PathSegment: Default
{
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl<'a, Value> HasRelativeAccess<'a> for RootVisitor<Value>
where Value: HasPathSegment + KnowsRelativeAccessType<'a>,
      Self: Into<Self::RelativeType> + HasPathSegment + HasRoot<'a> + HasGet<'a>,
      <Self as KnowsGetType<'a>>::GetType:
        Into<Self::RelativeType>
        + HasPathSegment<PathSegment = Self::PathSegment>,
      Value::RelativeType:
        HasRelativeAccess<'a>
      + KnowsRelativeAccessType<'a, RelativeType = Self::RelativeType>
      + HasPathSegment<PathSegment = <Self as HasPathSegment>::PathSegment>
      + HasParent<'a>
      + HasRoot<'a, Root = <Self as HasRoot<'a>>::Root>,
    <Value::RelativeType as KnowsParent<'a>>::Parent: Into<Self::RelativeType>
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
        where K: Into<<Self as HasPathSegment>::PathSegment>,
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            match segment.kind() {
                PathSegment::Root => Some(self.into()),
                PathSegment::Self_ | PathSegment::Super => self.relative(path),
                PathSegment::Other(_segment) => self.get(segment).and_then(|value| value.into().relative(path))
            }
        } else {
            Some(self.into())
        }
}
}

impl<'a, Value> HasPathSegment for RootVisitor<Value>
where Value: HasPathSegment
{
    type PathSegment = Value::PathSegment;
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }
}

impl<'a, Value> KnowsRelativeAccessType<'a> for RootVisitor<Value>
where Value: HasPathSegment + KnowsRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}

impl<Value> HasPath<Value::PathSegment> for RootVisitor<Value>
where Value: HasPathSegment
{
    fn path(&self) -> Path<Value::PathSegment>
    {
        let mut path = Path::default();
        path.segments.push(self.value.path_segment().clone());
        path
    }

}
