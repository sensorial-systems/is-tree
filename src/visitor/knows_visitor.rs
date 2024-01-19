use crate::{KnowsVisitor, Visitor};

impl<Parent, Value> KnowsVisitor<'_> for Visitor<Parent, Value> {
    type Visitor = Self;
}