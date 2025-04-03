//! This module defines the `KnowsVisitor` trait, which is used to define the associated type `Visitor` for a tree node.


//! A trait for objects that know their visitor.
pub trait KnowsVisitor {
    /// The visitor type.
    type Visitor;
    type VisitorMut;
}

impl<T> KnowsVisitor for Box<T>
where
    T: KnowsVisitor,
{
    type Visitor = T::Visitor;
    type VisitorMut = T::VisitorMut;
}

impl<T> KnowsVisitor for &T
where
    T: KnowsVisitor,
{
    type Visitor = T::Visitor;
    type VisitorMut = T::VisitorMut;
}

impl<T> KnowsVisitor for &mut T
where
    T: KnowsVisitor,
{
    type Visitor = T::Visitor;
    type VisitorMut = T::VisitorMut;
}