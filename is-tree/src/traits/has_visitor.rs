use crate::*;

pub trait KnowsVisitor<'a> {
    type Visitor: IsVisitor<'a>;
}

pub trait HasVisitor: Sized {
    fn visitor(self) -> RootVisitor<Self>;
}

impl<T> HasVisitor for &T {
    fn visitor(self) -> RootVisitor<Self> {
        RootVisitor::new(self)
    }
}

impl<T> HasVisitor for &mut T {
    fn visitor(self) -> RootVisitor<Self> {
        RootVisitor::new(self)
    }
}
