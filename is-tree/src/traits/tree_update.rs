use crate::{HasPathSegment, KnowsBranches, KnowsOwned, KnowsPathSegment};

pub trait AddBranch<'a>: KnowsBranches<'a> {
    fn add_branch(self, branch: impl Into<<Self::Branches as KnowsOwned>::Owned>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned;
}

pub trait HasGetOrCreate<'a>: AddBranch<'a>
{
    fn branch<PathSegment>(self, segment: PathSegment) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsPathSegment + KnowsOwned,
          PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment>;
}


// TODO: Deprecated. Remove this.
pub trait TreeUpdate<T>: HasPathSegment
where T: HasPathSegment + Sized
{
    fn add_branch(&mut self, _child: impl Into<T>) -> &mut T;

    /// Return `true`` if the branch was removed.
    fn remove_branch(&mut self, _identifier: &T::PathSegment) -> bool {
        false
    }
}
