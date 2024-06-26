//! Traits for relative path traversal.

use crate::{HasPathSegment, IsPathSegment, KnowsVisitor, PathSegment, UnsafeClone, HasParentMut, HasRootMut};

use super::{HasGet, HasParent, HasRoot};
use super::has_branches::HasBranches;

/// A trait for objects that have a relative path.
pub trait HasRelative<'a> {
    /// Gets a relative path.
    /// "self", "root", and "super" are reserved path segments.
    /// "self" is the current object, "root" is the root object, and "super" is the parent object.
    fn relative<K>(&'a self, path: impl IntoIterator<Item = K>) -> Option<Self>
    where K: Into<String>,
        Self: KnowsVisitor<Visitor = Self> + Clone + HasRoot + HasParent + HasPathSegment,
        &'a Self: HasBranches<Self>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            let visitor: Self = match segment.kind() {
                PathSegment::Self_ => self.clone(),
                PathSegment::Root => self.root(),
                PathSegment::Super => self.parent()?,
                PathSegment::Other(_) => self.get_impl::<Self>(segment)?
            };
            unsafe { crate::unsafe_::longer_ref(&visitor) }.relative(path)
        } else {
            Some(self.clone())
        }
    }
}

/// A trait for objects that have a relative path mutably.
/// By design, accessing a Visitor parent is unsafe.
pub trait HasRelativeMut<'a> {
    /// Gets a relative path mutably.
    /// "self", "root", and "super" are reserved path segments.
    /// "self" is the current object, "root" is the root object, and "super" is the parent object.
    unsafe fn relative_mut<K>(&'a mut self, path: impl IntoIterator<Item = K>) -> Option<Self>
    where K: Into<String>,
        Self: KnowsVisitor<VisitorMut = Self> + UnsafeClone + HasRootMut + HasParentMut + HasPathSegment + Sized,
        &'a mut Self: HasBranches<Self>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            let mut visitor: Self = match segment.kind() {
                PathSegment::Self_ => self.unsafe_clone(),
                PathSegment::Root => self.root_mut(),
                PathSegment::Super => self.parent_mut()?,
                PathSegment::Other(_) => self.get_impl::<Self>(segment)?
            };
            crate::unsafe_::longer_mut(&mut visitor).relative_mut(path)
        } else {
            Some(self.unsafe_clone())
        }
    }
}

impl<'a, T: Sized> HasRelative<'a> for T {}
impl<'a, T: Sized> HasRelativeMut<'a> for T {}