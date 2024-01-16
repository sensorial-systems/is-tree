use crate::{has_get::{KnowsGetType, HasGet}, RootVisitor, KnowsPathSegment, KnowsVisitor, IsVisitor, KnowsValue, KnowsParent, VisitorConstructor};

impl<Value> KnowsGetType for RootVisitor<Value>
where Value: KnowsGetType,
      Value::GetType: KnowsVisitor
{
    type GetType = <Value::GetType as KnowsVisitor>::Visitor;
}

// TODO: Remove this duplicity.
impl<'a, Value> KnowsGetType for &'a RootVisitor<Value>
where Value: KnowsGetType,
      Value::GetType: KnowsVisitor
{
    type GetType = <Value::GetType as KnowsVisitor>::Visitor;
}

impl<'a, Value> HasGet for &'a RootVisitor<Value>
where Value: Copy + HasGet,
      Value::GetType: KnowsPathSegment + KnowsVisitor,
      <Value::GetType as KnowsVisitor>::Visitor: KnowsPathSegment<PathSegment = <Value::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: VisitorConstructor<Value = Value::GetType> + KnowsParent + KnowsValue<Value = Value::GetType>,
      RootVisitor<Value>: Into<<Self::GetType as KnowsParent>::Parent> + 'a,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.value.get(key).map(|value| {
            self.visit(value)
        })
    }
}
