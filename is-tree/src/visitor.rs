//! Visitor pattern for tree traversal.

use crate::{longer_ref, HasBranches, HasParent, HasPath, HasPathSegment, HasRoot, KnowsVisitor, Path, UnsafeClone, UnsafeFrom, HasParentMut, HasRootMut};

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

unsafe impl<Parent, Value> UnsafeClone for Visitor<Parent, &mut Value>
where Parent: Clone
{
    unsafe fn unsafe_clone(&self) -> Self {
        let parent = self.parent.clone();
        let value = std::mem::transmute_copy(&self.value);
        Self { parent, value }
    }
}

unsafe impl<'a, Parent, Value> UnsafeClone for Visitor<Parent, &'a Value>
where Parent: Clone
{
    unsafe fn unsafe_clone(&self) -> Self {
        Self { parent: self.parent.clone(), value: self.value }
    }
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

unsafe impl<Parent, Value> HasParentMut for Visitor<Parent, Value>
where
    Self: KnowsVisitor,
    Self::VisitorMut: UnsafeFrom<Parent>,
    Parent: Clone
{
    unsafe fn parent_mut(&mut self) -> Option<Self::VisitorMut> {
        Some(Self::VisitorMut::unsafe_from(self.parent.clone()))
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

unsafe impl<Parent, Value> HasRootMut for Visitor<Parent, Value>
where
    Self: KnowsVisitor,
    Parent: HasRootMut,
    Parent::VisitorMut: Into<Self::VisitorMut>
{
    unsafe fn root_mut(&mut self) -> Self::VisitorMut {
        self.parent.root_mut().into()
    }
}

impl<'a, Parent, Value, T> HasBranches<T> for &'a Visitor<Parent, Value>
where
    Visitor<Parent, Value>: UnsafeClone,
    T: From<Visitor<Parent, Value>> + 'a,
    &'a T: HasBranches<T>
{
    fn branches_impl(self) -> impl Iterator<Item = T> {
        unsafe {
            let self_ = longer_ref(self); // TODO: Why is this necessary?
            let visitor = T::from(self_.unsafe_clone());
            longer_ref(&visitor).branches_impl()
        }
    }
}
