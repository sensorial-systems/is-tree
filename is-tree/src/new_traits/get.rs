use crate::new_traits::{HasBranches, HasBranchesAPI};

use crate::HasPathSegment;

pub trait HasGet {
    fn get<T>(self, segment: impl Into<String>) -> Option<T>
    where Self: HasBranches<T> + Sized,
          T: HasPathSegment
    {
        let segment = segment.into();
        self.branches::<T>().find(|value| value.path_segment() == &segment)
    }
}

impl<T> HasGet for T {}