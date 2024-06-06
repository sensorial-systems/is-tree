//! A trait for types that have a root.

use crate::KnowsVisitor;

/// A trait for types that have a root.
pub trait HasRoot: KnowsVisitor {
    /// Gets the root of the object.
    fn root(&self) -> Self::Visitor;
}

/// A trait for types that have a root mutably.
/// By design, accessing a Visitor parent is unsafe.
pub unsafe trait UnsafeHasRoot: Sized {
    /// Gets the root of the object.
    unsafe fn root_mut(&mut self) -> Self;
}

// impl<T> HasRoot for Box<T>
// where T: HasRoot
// {
//     fn root(&self) -> Self::Visitor {
//         self.as_ref().root()
//     }
// }
