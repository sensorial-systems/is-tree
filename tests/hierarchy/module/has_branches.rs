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

impl<'a> HasBranches<'a> for &'a mut Module {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        self.children.iter_mut()
    }
}

impl<'a> AddBranch<'a> for Module {
    fn add_branch(&mut self, branch: impl Into<Self::Branches>) -> &mut Self::Branches {
        self.children.push(branch.into());
        self.children.last_mut().unwrap()
    }
}
