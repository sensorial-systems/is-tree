//! This module contains the `HasGet` trait and its API trait `HasGetAPI`.

use crate::traits::has_branches::{HasBranches, HasBranchesAPI};

use crate::HasPathSegment;

/// This is the trait one should implement to provide a way to get branches by their path segments.
pub trait HasGet {
    /// Gets a branch by its path segment.
    /// It's discouraged to use this method directly. Instead, use the `get` and `get_mut` method from the `HasGetAPI` trait.
    fn get_impl<T>(self, segment: impl Into<String>) -> Option<T>
    where Self: HasBranches<T> + Sized,
          T: HasPathSegment
    {
        let segment = segment.into();
        self.branches_impl2::<T>().find(|value| value.path_segment() == segment)
    }
}

impl<T> HasGet for T {}

/// This is an API trait for getting branches by their path segments.
pub trait HasGetAPI<'a> {
    /// Gets a branch by its path segment.
    fn get<T>(&'a self, segment: impl Into<String>) -> Option<T>
    where &'a Self: HasGet + HasBranches<T>,
          T: HasPathSegment + 'a
    {
        self.get_impl::<T>(segment)
    }
}

impl<'a, T> HasGetAPI<'a> for T {}

pub trait HasGetMut<'a> {
    /// Gets a branch by its path segment mutably.
    fn get_mut<T>(&'a mut self, segment: impl Into<String>) -> Option<T>
    where &'a mut Self: HasGet + HasBranches<T>,
            T: HasPathSegment + 'a;
}
