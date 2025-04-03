use crate::longer_mut;
use super::{AddBranch, HasBranches, HasGet, HasGetMut, HasPathSegment};

// TODO: Add tests for this.
pub trait HasBranch<'a> {
    fn branch<T>(&'a mut self, segment: impl Into<String>) -> &'a mut T
    where &'a mut Self: HasGet + HasBranches<&'a mut T>,
          Self: AddBranch<T> + Sized + HasGetMut<'a>,
          T: HasPathSegment + 'a,
          String: Into<T>
    {
        let segment = segment.into();
        let self_ = unsafe { longer_mut(self) }; // This is safe.
        if let Some(value) = self.get_mut::<&mut T>(segment.clone()) {
            value
        } else {
            self_.add_branch(segment.into())
        }
    }
}

impl<'a, T> HasBranch<'a> for T {}