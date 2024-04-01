use crate::*;

impl<'a, Parent, Value> HasGet<'a> for &'a Visitor<Parent, Value>
where Parent: Clone,
      Value: Clone + HasGet<'a>,
      Value::Branches: HasPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: HasPathSegment,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      Visitor<Parent, Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent>,
{
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::Branches>
    where PathSegment: Into<String> {
        self
            .value.clone() // TODO: How to use .value() here instead?
            .get(segment)
            .map(|value| self.visit(value))
    }
}
