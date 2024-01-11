use crate::{Visitor, KnowsParentVisitor, HasValue};

pub trait IsVisitor<'a>: HasValue<'a> {
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>;
}
