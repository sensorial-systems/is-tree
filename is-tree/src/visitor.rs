//! Visitor pattern for tree traversal.

use crate::{HasPathSegment, Path, HasPath};

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
