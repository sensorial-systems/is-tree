//! Traits for objects that have a parent.

use crate::KnowsVisitor;

/// A trait for objects that have a parent.
pub trait HasParent: Sized + KnowsVisitor {
    /// Gets the parent of the object.
    fn parent(&self) -> Option<Self::Visitor>;
}

/// A trait for objects that have a parent mutably.
/// By design, accessing a Visitor parent is unsafe.
pub unsafe trait UnsafeHasParent: Sized + KnowsVisitor {
    /// Gets the parent of the object.
    unsafe fn parent_mut(&mut self) -> Option<Self::VisitorMut>;
}

impl<T> HasParent for Box<T>
where T: HasParent
{
    fn parent(&self) -> Option<Self::Visitor> {
        self.as_ref().parent()
    }
}

unsafe impl<T> UnsafeHasParent for Box<T>
where T: UnsafeHasParent
{
    unsafe fn parent_mut(&mut self) -> Option<Self::VisitorMut> {
        self.as_mut().parent_mut()
    }
}