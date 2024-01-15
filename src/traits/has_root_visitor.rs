use crate::{KnowsVisitor, RootVisitor};

pub trait HasRootVisitor {}

impl<'a, T: HasRootVisitor> KnowsVisitor for T {
    type Visitor = RootVisitor<T>;
}
