use crate::{Visitor, KnowsParentVisitor};

pub trait IsVisitor<'a> {
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>;
}
