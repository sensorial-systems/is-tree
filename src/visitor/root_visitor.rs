use super::*;

#[derive(Clone, Copy, Default)]
pub struct RootVisitor<Value> {
    pub value: Value
}

impl<'a, Value> KnowsParentVisitor<'a> for RootVisitor<Value>
where Value: KnowsPathSegment + KnowsParentVisitor<'a>
{
    type ParentVisitor = Self;
}

impl<'a, Value> IsVisitor<'a, Value> for RootVisitor<Value>
where Value: KnowsPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsPathSegment<PathSegment = Value::PathSegment>,
          Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}

impl<'a, Value> KnowsGetType<'a> for RootVisitor<Value>
where Value: KnowsPathSegment + KnowsGetType<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Value> HasGet<'a> for RootVisitor<Value>
where Value: Copy + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.value.get(key).map(|value| self.visit(value))
    }
}

impl<'a, Value> KnowsGetType<'a> for &'a RootVisitor<Value>
where Value: KnowsPathSegment + KnowsGetType<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Value> HasGet<'a> for &'a RootVisitor<Value>
where Value: Clone + Copy + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      RootVisitor<Value>: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.value.get(key).map(|value| (*self).clone().visit(value))
    }
}

impl<'a, Value> KnowsRoot<'a> for &'a RootVisitor<Value> {
    type Root = RootVisitor<Value>;
}

impl<'a, Value> HasRoot<'a> for &'a RootVisitor<Value>
where Value: Clone
{
    fn root(self) -> Self::Root {
        self.clone()
    }
}

impl<'a, Value> KnowsParent<'a> for &'a RootVisitor<Value> {
    type Parent = RootVisitor<Value>;
}

impl<'a, Value> HasParent<'a> for &'a RootVisitor<Value>
where Value: Clone
{
    fn parent(self) -> Self::Parent {
        self.clone()
    }
}

impl<'a, Value> RootVisitor<Value> {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl<'a, Value> HasRelativeAccess<'a> for &'a RootVisitor<Value>
where
    Value: Copy + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
    Value: KnowsRelativeAccessType<'a> + KnowsPathSegment + Clone + Copy + 'a,
    Self: Into<Self::RelativeType> + HasRoot<'a>,

    Self: HasGet<'a>,
    <Self as KnowsGetType<'a>>::GetType:
        KnowsParentVisitor<'a>
        + Into<Self::RelativeType>
        + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,
    RootVisitor<Value>: Into<<<Self as KnowsGetType<'a>>::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,

    &'a Self::RelativeType:
        HasRelativeAccess<'a,
            RelativeType = <Self as KnowsRelativeAccessType<'a>>::RelativeType,
            PathSegment = <Self as KnowsPathSegment>::PathSegment
        >
        + HasParent<'a>
        + HasRoot<'a>,
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
            Some(self.into())
        }
}
}

impl<'a, Value> KnowsPathSegment for RootVisitor<Value>
where Value: KnowsPathSegment
{
    type PathSegment = Value::PathSegment;
}

impl<'a, Value> HasPathSegment for RootVisitor<Value>
where Value: HasPathSegment
{
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }
}

impl<'a, Value> KnowsPathSegment for &'a RootVisitor<Value>
where Value: KnowsPathSegment
{
    type PathSegment = Value::PathSegment;
}

impl<'a, Value> HasPathSegment for &'a RootVisitor<Value>
where Value: HasPathSegment
{
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }
}

impl<'a, Value> KnowsRelativeAccessType<'a> for &'a RootVisitor<Value>
where Value: KnowsRelativeAccessType<'a>
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