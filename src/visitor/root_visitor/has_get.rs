use crate::{has_get::{KnowsGetType, HasGet}, RootVisitor, KnowsPathSegment, Visitor, KnowsParentVisitor, KnowsVisitor, IsVisitor, KnowsValue, KnowsParent, VisitorConstructor};

impl<'a, Value> KnowsGetType<'a> for RootVisitor<Value>
where Value: KnowsPathSegment + KnowsGetType<'a>,
      Value::GetType: KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasGet<'a> for RootVisitor<Value>
where Value: Copy + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsVisitor<'a> + KnowsPathSegment<PathSegment = Value::PathSegment>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>,
      Self::GetType: VisitorConstructor<'a, Owned = Self::GetType> + KnowsParent<'a> + KnowsValue<'a, Value = Value::GetType>,
      &'a Self: Into<<Self::GetType as KnowsParent<'a>>::Parent> + 'a,
      <<Value as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor: KnowsParent<'a>
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.value.get(key).map(|value|
            todo!()
            // self.visit(value)
        )
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
    fn get<K>(self, _key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        todo!()
        // self.value.get(key).map(|value| (*self).clone().visit(value))
    }
}
