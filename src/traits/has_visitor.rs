use crate::{new_visitor::{RootVisitor, HasRelativeAccessType, Visitor}, HasPathSegment, KnowsParentVisitor};

pub trait IsVisitor<'a, Value>
where Value: HasPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: HasPathSegment<PathSegment = Value::PathSegment>,
          Child: KnowsParentVisitor<'a>,
          Self: Into<Child::ParentVisitor>;
}

pub trait KnowsVisitor<'a, Value>
where Value: HasPathSegment
{
    type Visitor: IsVisitor<'a, Value>;
}

pub trait HasVisitor<'a, Value>: KnowsVisitor<'a, Value>
where Value: HasPathSegment
{
    fn visit(self) -> Self::Visitor;
}

pub trait HasRootVisitor
where Self: Sized + HasPathSegment
{}

impl<'a, T: HasRootVisitor> KnowsVisitor<'a, T> for T {
    type Visitor = RootVisitor<T>;
}

impl<'a, T> HasVisitor<'a, T> for T
where T: HasRootVisitor + HasPathSegment + HasRelativeAccessType<'a>,
      T::PathSegment: Default
{
    fn visit(self) -> Self::Visitor {
        Self::Visitor::new(self)
    }
}
