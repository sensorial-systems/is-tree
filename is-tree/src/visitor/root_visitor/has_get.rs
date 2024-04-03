use crate::*;

impl<'a, Value> HasGet<'a> for &'a RootVisitor<Value>
where Value: Clone + HasGet<'a>,
      Value::Branches: HasPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: HasPathSegment,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      RootVisitor<Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent> + Clone,
{
    fn get(self, segment: impl Into<String>) -> Option<Self::Branches> {
        self
            .value
            .clone()
            .get(segment)
            .map(|value| self.visit(value))
    }
}
