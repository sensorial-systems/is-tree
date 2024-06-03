//! Traits for objects that have a parent.

/// A trait for objects that have a parent.
pub trait HasParent: Sized {
    /// Gets the parent of the object.
    fn parent(&self) -> Option<Self>;
}

/// A trait for objects that have a parent mutably.
/// By design, accessing a Visitor parent is unsafe.
pub unsafe trait UnsafeHasParent: Sized {
    /// Gets the parent of the object.
    unsafe fn parent_mut(&mut self) -> Option<Self>;
}