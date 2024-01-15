use crate::{KnowsVisitor, Visitor};

impl<Parent, Value> KnowsVisitor for Visitor<Parent, Value> {
    type Visitor = Self;
}