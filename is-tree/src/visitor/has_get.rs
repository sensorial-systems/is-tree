use crate::*;

impl<'a, Parent, Value> HasGet<'a> for &'a Visitor<Parent, Value>
where Parent: Clone,
      Value: Clone + HasGet<'a>,
      Value::Branches: HasPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: HasPathSegment,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      Visitor<Parent, Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent>,
{
    fn get(self, segment: impl Into<String>) -> Option<Self::Branches> {
        self
            .value()
            .clone()
            .get(segment)
            .map(|value| self.visit(value))
    }
}
