use crate::{HasBranches, HasBranchesAPI};

use crate::HasPathSegment;

pub trait HasGet {
    fn get_impl<T>(self, segment: impl Into<String>) -> Option<T>
    where Self: HasBranches<T> + Sized,
          T: HasPathSegment
    {
        let segment = segment.into();
        self.branches::<T>().find(|value| value.path_segment() == segment)
    }
}

impl<T> HasGet for T {}

pub trait HasGetAPI<'a> {
    fn get<T>(&'a self, segment: impl Into<String>) -> Option<&'a T>
    where &'a Self: HasGet + HasBranches<&'a T>,
          &'a T: HasPathSegment + 'a
    {
        self.get_impl::<&T>(segment)
    }

    fn get_mut<T>(&'a mut self, segment: impl Into<String>) -> Option<&'a mut T>
    where &'a mut Self: HasGet + HasBranches<&'a mut T>,
          &'a mut T: HasPathSegment + 'a
    {
        self.get_impl::<&mut T>(segment)
    }
}

impl<'a, T> HasGetAPI<'a> for T {}