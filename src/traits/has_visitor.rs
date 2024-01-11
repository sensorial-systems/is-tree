use crate::{RootVisitor, KnowsPathSegment};

use crate::traits::*;

pub trait KnowsVisitor<'a, Value>
where Value: KnowsPathSegment
{
    type Visitor: IsVisitor<'a>;
}


pub trait HasVisitor<'a, Value>: KnowsVisitor<'a, Value>
where Value: KnowsPathSegment
{
    fn visit(self) -> Self::Visitor;
}

pub trait HasRootVisitor
where Self: Sized + KnowsPathSegment
{}

impl<'a, T: HasRootVisitor> KnowsVisitor<'a, T> for T {
    type Visitor = RootVisitor<T>;
}

impl<'a, T> HasVisitor<'a, T> for T
where T: HasRootVisitor + KnowsPathSegment + KnowsRelativeAccessType<'a>,
      T::PathSegment: Default
{
    fn visit(self) -> Self::Visitor {
        Self::Visitor::new(self)
    }
}
