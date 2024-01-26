use crate::*;

impl<'a, Value> KnowsBranches<'a> for RootVisitor<Value>
where Value: KnowsBranches<'a>,
      Value::Branches: KnowsVisitor<'a>
{
    type Branches = <Value::Branches as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasBranches<'a> for RootVisitor<Value>
where Value: Clone + HasBranches<'a>,
      Value::Branches: KnowsPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      Self: Into<<Self::Branches as KnowsParent<'a>>::Parent>,
{
    fn branches(&'a self) -> impl Iterator<Item = Self::Branches>
    {
        self
            .value
            .branches()
            .map(|value| self.visit(value))
    }
}
