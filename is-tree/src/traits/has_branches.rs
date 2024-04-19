use crate::HasPathSegment;


pub trait KnowsBranches<'a> {
    type Branches; // TODO: type Branches: KnowsOwned?
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

impl KnowsOwned for String {
    type Owned = String;
}

impl HasPathSegment for String {
    fn path_segment(&self) -> &String {
        self
    }
}