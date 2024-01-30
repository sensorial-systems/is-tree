use ::is_tree::*;
use super::super::*;

impl<'a> KnowsBranches<'a> for Library {
    type Branches = Module;
}

impl<'a> KnowsBranches<'a> for &'a Library {
    type Branches = &'a Module;
}

impl<'a> KnowsBranches<'a> for &'a mut Library {
    type Branches = &'a mut Module;
}

impl<'a> HasBranches<'a> for &'a Library {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        std::iter::once(&self.root_module)
    }
}
