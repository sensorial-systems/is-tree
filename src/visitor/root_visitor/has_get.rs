use crate::*;

impl<'a, Value> KnowsGetType<'a> for RootVisitor<Value>
where Value: KnowsGetType<'a>,
      Value::GetType: KnowsVisitor<'a>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasGet<'a> for RootVisitor<Value>
where Value: Clone + HasGet<'a>,
      Value::GetType: KnowsPathSegment + KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<'a, Value = Value::GetType>,
      Self: Into<<Self::GetType as KnowsParent<'a>>::Parent>,
{
    fn get<PathSegment>(&'a self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self
            .value
            .get(segment)
            .map(|value| self.visit(value))
    }
}
