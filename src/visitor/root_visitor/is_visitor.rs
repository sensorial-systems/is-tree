use crate::{IsVisitor, RootVisitor, Visitor, KnowsParentVisitor, KnowsValue, HasValue};

impl<'a, Value> KnowsValue<'a> for RootVisitor<Value> {
    type Value = Value;
}

impl<'a, Value> HasValue<'a> for RootVisitor<Value> {
    fn value(self) -> Self::Value {
        self.value
    }
}

impl<'a, Value> IsVisitor<'a> for RootVisitor<Value> {
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}
