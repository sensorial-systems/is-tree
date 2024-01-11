use crate::{IsVisitor, Visitor, KnowsParentVisitor, KnowsValue, HasValue};

impl<'a, Parent, Value> KnowsValue<'a> for &'a Visitor<Parent, Value> {
    type Value = Value;
}

impl<'a, Parent, Value> HasValue<'a> for &'a Visitor<Parent, Value>
where Value: Clone
{
    fn value(self) -> Self::Value {
        self.internal.value.clone()
    }
}

impl<'a, Parent, Value> IsVisitor<'a> for &'a Visitor<Parent, Value>
where Value: Clone
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}
