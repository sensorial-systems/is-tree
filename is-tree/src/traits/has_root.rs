//! A trait for types that have a root.

/// A trait for types that have a root.
pub trait HasRoot {
    /// Gets the root of the object.
    fn root(&self) -> Self;
}

/// A trait for types that have a root mutably.
/// By design, accessing a Visitor parent is unsafe.
pub unsafe trait UnsafeHasRoot: Sized {
    /// Gets the root of the object.
    unsafe fn root_mut(&mut self) -> Self;
}