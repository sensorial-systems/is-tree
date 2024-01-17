use crate::{has_get::{KnowsGetType, HasGet}, Visitor, KnowsPathSegment, KnowsVisitor, HasValue, KnowsParent, HasVisitorConstructor, IsVisitor};

impl<Parent, Value> KnowsGetType for Visitor<Parent, Value>
where Value: KnowsGetType,
      Value::GetType: KnowsVisitor
{
    type GetType = <Value::GetType as KnowsVisitor>::Visitor;
}

impl<Parent, Value> HasGet for Visitor<Parent, Value>
where Value: Clone + HasGet,
      Value::GetType: KnowsPathSegment + KnowsVisitor,
      <Value::GetType as KnowsVisitor>::Visitor: KnowsPathSegment<PathSegment = <Value::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<Value = Value::GetType>,
      Self: Into<<Self::GetType as KnowsParent>::Parent> + Clone,
{
    fn get<K>(&self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self
            .value()
            .get(key)
            .map(|value| self.visit(value))

    }
}
