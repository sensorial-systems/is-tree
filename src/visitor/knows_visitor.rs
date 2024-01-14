use crate::{KnowsVisitor, Visitor};

impl<'a, Parent, Value> KnowsVisitor<'a> for Visitor<Parent, Value> {
    type Visitor = Self;
}