use crate::traits::has_branches::{HasBranches, HasBranchesAPI};

use crate::HasPathSegment;

pub trait HasGet {
    fn get_impl<T>(self, segment: impl Into<String>) -> Option<T>
    where Self: HasBranches<T> + Sized,
          T: HasPathSegment
    {
        let segment = segment.into();
        self.branches_impl2::<T>().find(|value| value.path_segment() == segment)
    }
}

impl<T> HasGet for T {}

pub trait HasGetAPI<'a> {
    fn get<T>(&'a self, segment: impl Into<String>) -> Option<T>
    where &'a Self: HasGet + HasBranches<T>,
          T: HasPathSegment + 'a
    {
        self.get_impl::<T>(segment)
    }

    fn get_mut<T>(&'a mut self, segment: impl Into<String>) -> Option<T>
    where &'a mut Self: HasGet + HasBranches<T>,
          T: HasPathSegment + 'a
    {
        self.get_impl::<T>(segment)
    }
}

impl<'a, T> HasGetAPI<'a> for T {}