use crate::*;

impl<'a, Parent, Value> KnowsBranches<'a> for Visitor<Parent, Value>
where Value: KnowsBranches<'a>,
      Value::Branches: KnowsVisitor<'a>
{
    type Branches = <<Value as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Parent, Value> HasBranches<'a> for &'a Visitor<Parent, Value>
where Parent: Clone,
      Value: Clone + HasBranches<'a>,
      Value::Branches: KnowsPathSegment + KnowsVisitor<'a>,
      <Value::Branches as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::Branches as KnowsPathSegment>::PathSegment>,
      Self::Branches: HasVisitorConstructor<'a, Value = Value::Branches>,
      Visitor<Parent, Value>: Into<<Self::Branches as KnowsParent<'a>>::Parent> + Clone,
{
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        self
            .value // TODO: How to use .value() here instead?
            .clone() 
            .branches()
            .map(|value| self.visit(value))
    }
}
