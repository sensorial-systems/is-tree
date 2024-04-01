use ::is_tree::*;
use super::super::*;

impl<'a> HasBranches<'a> for &'a Library {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        std::iter::once(&self.root_module)
    }
}

impl<'a> HasBranches<'a> for &'a mut Library {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        std::iter::once(&mut self.root_module)
    }
}

impl<'a> AddBranch<'a> for Library
where Self::Branches: KnowsOwned<Owned = Module>
{
    fn add_branch(&'a mut self, branch: impl Into<Module>) -> &'a mut Module
    where Self::Branches: KnowsOwned {
        self.root_module = branch.into();
        &mut self.root_module
    }
}
