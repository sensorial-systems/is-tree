use crate::traits::*;

pub trait KnowsVisitor {
    type Visitor: IsVisitor;
}

pub trait HasVisitor: KnowsVisitor {
    fn visit(self) -> Self::Visitor;
}

impl<T> HasVisitor for T
where T: HasRootVisitor
{
    fn visit(self) -> Self::Visitor {
        Self::Visitor::new(self)
    }
}
