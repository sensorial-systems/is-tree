use crate::{new_visitor::RootVisitor, HasPathSegment};

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
    type Visitor = RootVisitor<T>;
}

impl<T> HasVisitor for T
where T: HasRootVisitor + HasPathSegment,
      T::PathSegment: Default
{
    fn visit(self) -> Self::Visitor {
        Self::Visitor::new(self)
    }
}
