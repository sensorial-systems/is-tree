use crate::{HasPathSegment, KnowsBranches, KnowsPathSegment};

pub trait AddBranch<'a>: KnowsBranches<'a> {
    fn add_branch(&mut self, branch: impl Into<Self::Branches>) -> &mut Self::Branches;
}

pub trait HasGetOrCreate<'a>: AddBranch<'a> {
    fn branch<PathSegment>(&mut self, segment: PathSegment) -> &mut Self::Branches
    where Self::Branches: KnowsPathSegment,
          PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment>,
          <Self::Branches as KnowsPathSegment>::PathSegment: Into<Self::Branches>;
}

pub trait TreeUpdate<T>: HasPathSegment
where T: HasPathSegment + Sized
{
    fn add_branch(&mut self, _child: impl Into<T>) -> &mut T;

    /// Return `true`` if the branch was removed.
    fn remove_branch(&mut self, _identifier: &T::PathSegment) -> bool {
        false
    }
}
