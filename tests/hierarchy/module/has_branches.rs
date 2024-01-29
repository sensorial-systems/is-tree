use ::is_tree::*;

use super::super::*;

impl<'a> KnowsBranches<'a> for Module {
    type Branches = &'a Module;
}

impl<'a> HasBranches<'a> for &'a Module {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        self.children.iter()
    }
}
