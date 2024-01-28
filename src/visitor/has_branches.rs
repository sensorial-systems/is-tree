use crate::*;

impl<'a, Parent, Value> KnowsBranches<'a> for Visitor<Parent, Value>
where &'a Value: KnowsBranches<'a> + 'a,
      <&'a Value as KnowsBranches<'a>>::Branches: KnowsVisitor<'a>
{
    type Branches = <<&'a Value as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Parent, Value> HasBranches<'a> for &'a Visitor<Parent, Value>
where Parent: Clone,
      &'a Value: Clone + HasBranches<'a>,
      <&'a Value as KnowsBranches<'a>>::Branches: KnowsPathSegment + KnowsVisitor<'a>,
      <<&'a Value as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <<&'a Value as KnowsBranches<'a>>::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = <&'a Value as KnowsBranches<'a>>::Branches>,
      Visitor<Parent, Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent> + Clone,
{
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        self
            .internal.value // TODO: How to use .value() here instead?
            .branches()
            .map(|value| self.visit(value))
    }
}
