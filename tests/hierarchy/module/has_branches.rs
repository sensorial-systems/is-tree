use ::is_tree::*;

use super::super::*;

impl<'a> KnowsBranches<'a> for Module {
    type Branches = Module;
}

impl<'a> KnowsBranches<'a> for &'a Module {
    type Branches = &'a Module;
}

impl<'a> KnowsBranches<'a> for &'a mut Module {
    type Branches = &'a mut Module;
}

impl<'a> HasBranches<'a> for &'a Module {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        self.children.iter()
    }
}
