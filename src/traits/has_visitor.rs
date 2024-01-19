use crate::*;

pub trait KnowsVisitor<'a> {
    type Visitor: IsVisitor<'a>;
}

pub trait HasVisitor {
    fn visitor(&self) -> RootVisitor<&Self>;
}

impl<T> HasVisitor for T {
    fn visitor(&self) -> RootVisitor<&Self> {
        RootVisitor::new(self)
    }
}
