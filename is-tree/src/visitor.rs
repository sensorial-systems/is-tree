//! Visitor pattern for tree traversal.

use crate::{longer_mut, longer_ref, HasBranches, HasParent, HasPath, HasPathSegment, HasRoot, KnowsVisitor, Path, UnsafeClone, UnsafeFrom, UnsafeHasParent, UnsafeHasRoot};

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

impl<Parent, Value> HasParent for Visitor<Parent, Value>
where
    Self: KnowsVisitor,
    Parent: Clone + Into<Self::Visitor>
{
    fn parent(&self) -> Option<Self::Visitor> {
        Some(self.parent.clone().into())
    }
}

unsafe impl<Parent, Value> UnsafeHasParent for Visitor<Parent, Value>
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

impl<'a, Parent: KnowsVisitor, Value> HasBranches<Parent::Visitor> for &'a Visitor<Parent, Value>
where
    Visitor<Parent, Value>: Clone + Into<Parent::Visitor>,
    &'a Parent::Visitor: HasBranches<Parent::Visitor>
{
    fn branches_impl(self) -> impl Iterator<Item = Parent::Visitor> {
        let self_ = unsafe { longer_ref(self) }; // TODO: Why is this necessary?
        let visitor = self_.clone().into();
        unsafe { longer_ref(&visitor) }.branches_impl()
    }
}

impl<'a, Parent, Value, T> HasBranches<T> for &'a mut Visitor<Parent, Value>
where
    Visitor<Parent, Value>: UnsafeClone,
    T: From<Visitor<Parent, Value>> + 'a,
    &'a mut T: HasBranches<T>
{
    fn branches_impl(self) -> impl Iterator<Item = T> {
        unsafe {
            let visitor = self.unsafe_clone();
            let mut visitor = T::from(visitor);
            longer_mut(&mut visitor).branches_impl()
        }
    }
}
