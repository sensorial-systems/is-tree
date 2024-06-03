//! Traits for types that can be used as path segments.

use std::hash::Hash;

use crate::PathSegment;

/// A trait for types that can be used as path segments.
pub trait IsPathSegment: PartialEq + Eq + Hash + Clone {
    /// Gets the root path segment.
    fn root() -> Self;
    /// Gets the self path segment.
    fn self_() -> Self;
    /// Gets the super path segment.
    fn super_() -> Self;
    /// Gets the kind of path segment.
    fn kind(&self) -> PathSegment<&Self> {
        if Self::root().eq(self) {
            PathSegment::Root
        } else if Self::self_().eq(self) {
            PathSegment::Self_
        } else if Self::super_().eq(self) {
            PathSegment::Super
        } else {
            PathSegment::Other(self)
        }
    }
}

impl IsPathSegment for String {
    fn root() -> Self {
        "root".to_string()
    }
    fn self_() -> Self {
        "self".to_string()
    }
    fn super_() -> Self {
        "super".to_string()
    }
}
