use crate::{KnowsBranches, KnowsOwned};

pub trait AddBranch<'a>: KnowsBranches<'a> {
    fn add_branch(self, branch: impl Into<<Self::Branches as KnowsOwned>::Owned>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned;
}

pub trait HasGetOrCreate<'a>: AddBranch<'a>
{
    fn branch(self, segment: impl Into<String>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned;
}


