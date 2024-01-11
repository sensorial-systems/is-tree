use crate::{Visitor, KnowsParentVisitor};

pub trait IsVisitor<'a> {
    type Value;

    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>;

    fn value(self) -> Self::Value;
}
