use crate::*;

pub trait KnowsVisitor<'a> {
    type Visitor: IsVisitor<'a>;
}

pub trait HasVisitor {
    fn visitor(&self) -> RootVisitor<&Self>;

    fn visitor_mut(&mut self) -> RootVisitor<&mut Self>;
}

impl<T> HasVisitor for T {
    fn visitor(&self) -> RootVisitor<&Self> {
        RootVisitor::new(self)
    }

    fn visitor_mut(&mut self) -> RootVisitor<&mut Self> {
        RootVisitor::new(self)
    }
}

impl<T> From<T> for RootVisitor<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}