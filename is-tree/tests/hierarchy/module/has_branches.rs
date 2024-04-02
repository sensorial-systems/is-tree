use ::is_tree::*;

use super::super::*;

impl<'a> AddBranch<'a> for Module
where Self::Branches: KnowsOwned<Owned = Module>
{
    fn add_branch(&'a mut self, branch: impl Into<Module>) -> &'a mut Module
    where Self::Branches: KnowsOwned {
        self.children.push(branch.into());
        self.children.last_mut().unwrap()
    }
}
