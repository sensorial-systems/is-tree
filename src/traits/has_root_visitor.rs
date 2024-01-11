use crate::{KnowsVisitor, RootVisitor};

pub trait HasRootVisitor {}

impl<'a, T: HasRootVisitor> KnowsVisitor<'a> for T {
    type Visitor = RootVisitor<T>;
}
