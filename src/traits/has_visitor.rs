use crate::{traits::*, RootVisitor};

pub trait KnowsVisitor {
    type Visitor: IsVisitor;
}

pub trait HasVisitor {
    fn visitor(&self) -> RootVisitor<&Self>;
}

impl<T> HasVisitor for T {
    fn visitor(&self) -> RootVisitor<&Self> {
        RootVisitor::new(self)
    }
}
