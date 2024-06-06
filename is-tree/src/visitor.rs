//! Visitor pattern for tree traversal.

use crate::{HasParent, HasPath, HasPathSegment, HasRoot, KnowsVisitor, Path, UnsafeHasRoot};

/// A visitor for tree traversal.
#[derive(Clone, Debug, Default)]
pub struct Visitor<Parent, Value> {
    /// The parent of the visitor.
    pub parent: Parent,
    /// The value of the visitor.
    pub value: Value
}

impl<Parent, Value> Visitor<Parent, Value> {
    /// Creates a new visitor.
    pub fn new(parent: Parent, value: Value) -> Self {
        Self { parent, value }
    }
}

impl<Parent, Value> HasPathSegment for Visitor<Parent, Value>
where Value: HasPathSegment
{
    fn path_segment(&self) -> String {
        self.value.path_segment()
    }
}

impl<Parent, Value> HasPath for Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: HasPath
{
    fn path(&self) -> Path
    {
        let mut path = self.parent.path();
        path.segments.push(self.value.path_segment());
        path
    }
}

// Parent as Visitor is a convention because it's always a Box<T> where T is an enumeration visitor defined with `visitor!`.
impl<Parent, Value> KnowsVisitor for Visitor<Parent, Value>
where Parent: KnowsVisitor
{
    type Visitor = Parent::Visitor;
    type VisitorMut = Parent::VisitorMut;
}

impl<Parent, Value> HasParent for Visitor<Parent, Value>
where
    Self: KnowsVisitor,
    Parent: Clone + Into<Self::Visitor>
{
    fn parent(&self) -> Option<Self::Visitor> {
        Some(self.parent.clone().into())
    }
}

impl<Parent, Value> HasRoot for Visitor<Parent, Value>
where
    Self: KnowsVisitor,
    Parent: HasRoot,
    Parent::Visitor: Into<Self::Visitor>
{
    fn root(&self) -> Self::Visitor {
        self.parent.root().into()
    }
}

unsafe impl<Parent, Value> UnsafeHasRoot for Visitor<Parent, Value>
where
    Self: KnowsVisitor,
    Parent: UnsafeHasRoot,
    Parent::VisitorMut: Into<Self::VisitorMut>
{
    unsafe fn root_mut(&mut self) -> Self::VisitorMut {
        self.parent.root_mut().into()
    }
}