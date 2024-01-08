use crate::HasPathSegment;

pub trait TreeUpdate<T>: HasPathSegment
where T: HasPathSegment + Sized
{
    fn add_branch(&mut self, _child: impl Into<T>) -> &mut T;

    /// Return `true`` if the branch was removed.
    fn remove_branch(&mut self, _identifier: &T::PathSegment) -> bool {
        false
    }
}
