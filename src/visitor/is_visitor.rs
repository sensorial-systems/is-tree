use crate::{IsVisitor, Visitor, KnowsParentVisitor};

impl<'a, Parent, Value> IsVisitor<'a> for &'a Visitor<Parent, Value>
where Value: Clone
{
    type Value = Value;
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }

    fn value(self) -> Self::Value {
        self.internal.clone().value.clone()
    }
}
