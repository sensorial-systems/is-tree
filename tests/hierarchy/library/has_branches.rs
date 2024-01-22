use ::is_tree::*;
use super::super::*;

impl<'a> KnowsBranches<'a> for Library {
    type Branches = &'a Module;
}

impl<'a> HasBranches<'a> for Library {
    fn branches(&'a self) -> impl Iterator<Item = Self::Branches> {
        std::iter::once(&self.root_module)
    }
}