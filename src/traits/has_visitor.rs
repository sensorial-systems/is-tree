use crate::{visitor::{RootVisitor, Visitor}, KnowsPathSegment, KnowsParentVisitor};

use crate::traits::*;

pub trait IsVisitor<'a, Value>
where Value: KnowsPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: KnowsPathSegment<PathSegment = Value::PathSegment>,
          Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>;
}

pub trait KnowsVisitor<'a, Value>
where Value: KnowsPathSegment
{
    type Visitor: IsVisitor<'a, Value>;
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
