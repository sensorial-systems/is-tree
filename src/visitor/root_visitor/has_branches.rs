use crate::*;

impl<'a, Value> KnowsBranches<'a> for RootVisitor<Value>
where &'a Value: KnowsBranches<'a> + 'a,
      <&'a Value as KnowsBranches<'a>>::Branches: KnowsVisitor<'a>
{
    type Branches = <<&'a Value as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasBranches<'a> for &'a RootVisitor<Value>
where &'a Value: Clone + HasBranches<'a>,
      <&'a Value as KnowsBranches<'a>>::Branches: KnowsPathSegment + KnowsVisitor<'a>,
      <<&'a Value as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <<&'a Value as KnowsBranches<'a>>::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = <&'a Value as KnowsBranches<'a>>::Branches>,
      RootVisitor<Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent> + Clone,
{
    fn branches(self) -> impl Iterator<Item = Self::Branches>
    {
        self
            .value
            .branches()
            .map(|value| self.visit(value))
    }
}
