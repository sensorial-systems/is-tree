use crate::{IsVisitor, RootVisitor, Visitor, KnowsParentVisitor};

impl<'a, Value> IsVisitor<'a> for RootVisitor<Value> {
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}
