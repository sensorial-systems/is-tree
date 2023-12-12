use crate::{new_visitor::{RootVisitor, Visitor}, HasPathSegment};

pub trait IsVisitor {}

pub trait KnowsVisitor {
    type Visitor: IsVisitor;
}

pub trait HasVisitor: KnowsVisitor {
    fn visit(self) -> Self::Visitor;
}

pub trait HasRootVisitor
where Self: Sized + HasPathSegment
{}

impl<T: HasRootVisitor> KnowsVisitor for T {
    type Visitor = Visitor<RootVisitor, Self>;
}

impl<T: HasRootVisitor> HasVisitor for T {
    fn visit(self) -> Self::Visitor {
        Self::Visitor::new(self)
    }
}
