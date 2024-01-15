use crate::{KnowsVisitor, Visitor};

impl<'a, Parent, Value> KnowsVisitor for Visitor<Parent, Value> {
    type Visitor = Self;
}