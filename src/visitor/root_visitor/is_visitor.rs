use crate::{IsVisitor, RootVisitor, KnowsPathSegment, Visitor, KnowsParentVisitor};

impl<'a, Value> IsVisitor<'a, Value> for RootVisitor<Value>
where Value: KnowsPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsPathSegment<PathSegment = Value::PathSegment>,
          Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}
