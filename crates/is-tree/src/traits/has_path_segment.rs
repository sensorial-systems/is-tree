//! Traits for types that have a path segment.

use crate::Path;

/// This trait should be implemented by types that have an absolute path.
pub trait HasPath {
    /// Gets the path of the type.
    fn path(&self) -> Path;
}

impl HasPath for () {
    fn path(&self) -> Path {
        Path::default()
    }
}

impl<T> HasPath for Box<T>
where
    T: HasPath,
{
    fn path(&self) -> Path {
        (**self).path()
    }
}

/// This trait should be implemented by types that have a path segment.
pub trait HasPathSegment {
    /// Gets the path segment of the type.
    fn path_segment(&self) -> String;

    /// Checks if the type is identified by the given path segment.
    fn is(&self, identifier: impl PartialEq<String>) -> bool {
        identifier.eq(&self.path_segment())
    }
}

impl<T: HasPathSegment> HasPathSegment for &T {
    fn path_segment(&self) -> String {
        (*self).path_segment()
    }
}

impl<T: HasPathSegment> HasPathSegment for &mut T {
    fn path_segment(&self) -> String {
        (**self).path_segment()
    }
}

impl HasPathSegment for String {
    fn path_segment(&self) -> String {
        self.clone()
    }
}