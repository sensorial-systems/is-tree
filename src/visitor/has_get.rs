use crate::*;

impl<'a, Parent, Value> KnowsGetType<'a> for Visitor<Parent, Value>
where Value: KnowsGetType<'a>,
      Value::GetType: KnowsVisitor<'a>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Parent, Value> HasGet<'a> for &'a Visitor<Parent, Value>
where Parent: Clone,
      Value: Clone + HasGet<'a>,
      Value::GetType: KnowsPathSegment + KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<'a, Value = Value::GetType>,
      Visitor<Parent, Value>: Into<<Self::GetType as KnowsParent<'a>>::Parent>,
{
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self
            .value.clone() // TODO: How to use .value() here instead?
            .get(segment)
            .map(|value| self.visit(value))
    }
}
