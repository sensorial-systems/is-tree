mod visitor;
use is_tree::IsTree;
pub use visitor::*;

mod add_branch;

use super::{Visitors, Library};

#[derive(IsTree)]
#[tree(visitor = "ModuleVisitor<'a, &'a Module>")]
#[tree(relative_visitor = "Visitors<'a, &'a Library, &'a Module>")]
#[tree(type_iterator = "String")]
pub struct Module {
    #[tree(path_segment)]
    pub name: String,
    #[tree(branch)]
    pub children: Vec<Module>
}

impl From<String> for Module {
    fn from(name: String) -> Self {
        let children = Default::default();
        Self { name, children }
    }
}
