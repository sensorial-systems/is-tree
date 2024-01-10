use crate::{has_get::{KnowsGetType, HasGet}, RootVisitor, KnowsPathSegment, Visitor, KnowsParentVisitor, IsVisitor};

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
where Value: Copy + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      RootVisitor<Value>: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.value.get(key).map(|value| (*self).clone().visit(value))
    }
}
