use crate::*;

impl<'a, Value> KnowsBranches<'a> for RootVisitor<Value>
where Value: ToOwned,
      Value::Owned: KnowsBranches<'a>,
      <Value::Owned as KnowsBranches<'a>>::Branches: KnowsVisitor<'a>
{
    type Branches = <<<Value as ToOwned>::Owned as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasBranches<'a> for RootVisitor<Value>
where Value: Clone + ToOwned + HasBranches<'a, Branches = <Value::Owned as KnowsBranches<'a>>::Branches>,
      Value::Owned: KnowsBranches<'a>,
      <Value::Owned as KnowsBranches<'a>>::Branches: KnowsPathSegment + KnowsVisitor<'a>,
      <<Value::Owned as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <<<Value as ToOwned>::Owned as KnowsBranches<'a>>::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = <<Value as ToOwned>::Owned as KnowsBranches<'a>>::Branches>,
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
