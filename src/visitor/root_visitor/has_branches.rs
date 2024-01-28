use crate::*;

impl<'a, Value> KnowsBranches<'a> for RootVisitor<Value>
where Value: KnowsBranches<'a> + 'a,
      <Value as KnowsBranches<'a>>::Branches: KnowsVisitor<'a>
{
    type Branches = <Value::Branches as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasBranches<'a> for &'a RootVisitor<Value>
where Value: Clone + HasBranches<'a>,
      Value::Branches: KnowsPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      RootVisitor<Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent> + Clone,
{
    fn branches(self) -> impl Iterator<Item = Self::Branches>
    {
        self
            .value
            .clone()
            .branches()
            .map(|value| self.visit(value))
    }
}
