use crate::*;

impl<'a, Value> HasGet<'a> for &'a RootVisitor<Value>
where Value: Clone + HasGet<'a>,
      Value::Branches: HasPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: HasPathSegment<PathSegment = <Value::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      RootVisitor<Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent> + Clone,
{
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::Branches>
    where PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment> {
        self
            .value
            .clone()
            .get(segment)
            .map(|value| self.visit(value))
    }
}
