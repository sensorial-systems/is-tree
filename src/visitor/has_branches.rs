use crate::*;

impl<'a, Parent, Value> KnowsBranches<'a> for Visitor<Parent, Value>
where Value: KnowsBranches<'a>,
      Value::Branches: KnowsVisitor<'a>
{
    type Branches = <Value::Branches as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Parent, Value> HasBranches<'a> for Visitor<Parent, Value>
where Parent: Clone,
      Value: Clone + HasBranches<'a>,
      Value::Branches: KnowsPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      Self: Into<<Self::Branches as KnowsParent<'a>>::Parent>,
{
    fn branches(&'a self) -> impl Iterator<Item = Self::Branches> {
        self
            .internal.value // TODO: How to use .value() here instead?
            .branches()
            .map(|value| self.visit(value))
    }
}
