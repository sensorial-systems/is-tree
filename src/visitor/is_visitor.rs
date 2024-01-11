use crate::{IsVisitor, KnowsPathSegment, Visitor, KnowsParentVisitor};

impl<'a, Parent, Value> IsVisitor<'a, Value> for &'a Visitor<Parent, Value>
where Value: KnowsPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
        where Child: KnowsPathSegment<PathSegment = <Value as KnowsPathSegment>::PathSegment>,
              Child: KnowsParentVisitor<'a>,
              Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}
