use crate::{KnowsParentVisitor, RootVisitor};

impl<'a, Value> KnowsParentVisitor<'a> for RootVisitor<Value> {
    type ParentVisitor = Self;
}
