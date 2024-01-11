use crate::{has_get::{KnowsGetType, HasGet}, Visitor, KnowsParentVisitor, KnowsPathSegment, KnowsVisitor};

impl<'a, Parent, Value> KnowsGetType<'a> for Visitor<Parent, Value>
where Value: KnowsGetType<'a>,
      Value::GetType: KnowsParentVisitor<'a>
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Parent, Value> HasGet<'a> for Visitor<Parent, Value>
where Value: Clone + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.internal.value.clone().get(key).map(|value| Visitor::new_with_parent(self.into(), value))
    }
}

impl<'a, Parent, Value> KnowsGetType<'a> for &'a Visitor<Parent, Value>
where Value: KnowsGetType<'a> + KnowsPathSegment,
      Value::GetType: KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Parent, Value> HasGet<'a> for &'a Visitor<Parent, Value>
where Value: Clone + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>,
{
    fn get<K>(self, _key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        todo!()
        // self.internal.value.clone().get(key).map(|value| self.visit(value))
    }
}
