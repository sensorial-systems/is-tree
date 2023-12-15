use super::*;

pub type RootVisitor<Value> = Visitor<(), Value>;

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
where Value: HasPathSegment + HasRelativeAccessType<'a>, Value::PathSegment: Default
{
    pub fn new(value: Value) -> Self {
        let parent = Default::default();
        Self { parent, value }
    }
}

impl<'a, Value> HasRelativeAccess<'a> for RootVisitor<Value>
where Value: HasPathSegment + HasRelativeAccessType<'a>,
      Self: Into<Self::RelativeType>,

      Value::RelativeType:
      HasRelativeAccess<'a>
    + HasRelativeAccessType<'a, RelativeType = Self::RelativeType>
    + HasPathSegment<PathSegment = Self::PathSegment>
    + HasParent<'a>
    + HasRoot<'a, Root = Self::Root>,
    <Value::RelativeType as KnowsParent<'a>>::Parent: Into<Self::RelativeType>
{
    fn relative<K>(self, _path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
        where K: Into<<Self as HasPathSegment>::PathSegment>,
    {
        Some(self.into())
    }
}
