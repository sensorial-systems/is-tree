
pub trait KnowsBranches<'a> {
    type Branches;
}

pub trait HasBranches<'a>: KnowsBranches<'a> {
    fn branches(self) -> impl Iterator<Item = Self::Branches>;
}

