mod visitor;
use is_tree::IsTree;
pub use visitor::*;

mod add_branch;

use super::{Visitors, Library};

#[derive(IsTree)]
#[tree(visitor = "ModuleVisitor<&'a Library, &'a Module>")]
#[tree(relative_visitor = "Visitors<&'a Library, &'a Module>")]
pub struct Module {
    #[tree(path_segment)]
    #[tree(type_iterator = "String")]
    pub name: String,
    #[tree(branch)]
    #[tree(type_iterator = "String")]
    pub children: Vec<Module>
}

impl From<String> for Module {
    fn from(name: String) -> Self {
        let children = Default::default();
        Self { name, children }
    }
}
