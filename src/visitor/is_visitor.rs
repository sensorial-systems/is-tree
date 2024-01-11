use crate::{IsVisitor, Visitor, KnowsParentVisitor};

impl<'a, Parent, Value> IsVisitor<'a> for &'a Visitor<Parent, Value> {
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}
