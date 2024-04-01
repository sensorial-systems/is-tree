use crate::{HasPathSegment, KnowsBranches, KnowsOwned};

pub trait AddBranch<'a>: KnowsBranches<'a> {
    fn add_branch(self, branch: impl Into<<Self::Branches as KnowsOwned>::Owned>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned;
}

pub trait HasGetOrCreate<'a>: AddBranch<'a>
{
    fn branch(self, segment: impl Into<String>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned;
}


// TODO: Deprecated. Remove this.
pub trait TreeUpdate<T>: HasPathSegment
where T: HasPathSegment + Sized
{
    fn add_branch(&mut self, _child: impl Into<T>) -> &mut T;

    /// Return `true`` if the branch was removed.
    fn remove_branch(&mut self, _identifier: &String) -> bool {
        false
    }
}
