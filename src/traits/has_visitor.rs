use crate::{traits::*, RootVisitor};

pub trait KnowsVisitor {
    type Visitor: IsVisitor;
}

impl<T: HasRootVisitor> KnowsVisitor for T {
    type Visitor = RootVisitor<T>;
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
