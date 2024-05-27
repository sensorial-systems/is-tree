use ::is_tree::*;
use super::super::*;

impl<'a> AddBranch<'a> for Library {
    fn add_branch(&'a mut self, branch: impl Into<Module>) -> &'a mut Module {
        self.root_module = branch.into();
        &mut self.root_module
    }
}
