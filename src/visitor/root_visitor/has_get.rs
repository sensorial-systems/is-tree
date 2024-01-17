use crate::{has_get::{KnowsGetType, HasGet}, RootVisitor, KnowsPathSegment, KnowsVisitor, IsVisitor, KnowsParent, HasVisitorConstructor, HasValue};

impl<Value> KnowsGetType for RootVisitor<Value>
where Value: KnowsGetType,
      Value::GetType: KnowsVisitor
{
    type GetType = <Value::GetType as KnowsVisitor>::Visitor;
}

impl<Value> HasGet for RootVisitor<Value>
where Value: Clone + HasGet,
      Value::GetType: KnowsPathSegment + KnowsVisitor,
      <Value::GetType as KnowsVisitor>::Visitor: KnowsPathSegment<PathSegment = <Value::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<Value = Value::GetType>,
      Self: Into<<Self::GetType as KnowsParent>::Parent>,
{
    fn get<K>(&self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self
            .value()
            .get(key)
            .map(|value| self.visit(value))
    }
}
