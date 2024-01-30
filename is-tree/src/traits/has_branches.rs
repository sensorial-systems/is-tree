
pub trait KnowsBranches<'a> {
    type Branches;
}

pub trait HasBranches<'a>: KnowsBranches<'a> {
    fn branches(self) -> impl Iterator<Item = Self::Branches>;
}

// TODO: Move it to its own module file.
pub trait KnowsOwned {
    type Owned;
}

impl<T> KnowsOwned for &T {
    type Owned = T;
}

impl<T> KnowsOwned for &mut T {
    type Owned = T;
}