use ::is_tree::*;
use super::super::*;

impl<'a> KnowsBranches<'a> for Library {
    type Branches = &'a Module;
}

impl<'a> HasBranches<'a> for &'a Library {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        std::iter::once(&self.root_module)
    }
}

// FIXME: This is a workaround for an unsolved constraint.
impl<'a> HasBranches<'a> for &'a &'a Library {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        std::iter::once(&self.root_module)
    }
}