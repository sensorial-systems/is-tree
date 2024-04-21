mod visitor;
use is_tree::IsTree;
pub use visitor::*;

mod has_branches;

use super::{Module, Visitors};

#[derive(IsTree)]
#[tree(branches = "Module")]
#[tree(visitor = "Visitors<'a, &'a Library, &'a Module>")]
#[tree(relative_visitor = "Visitors<'a, &'a Library, &'a Module>")]
#[tree(type_iterator = "String")]
pub struct Library {
    #[tree(path_segment)]
    pub name: String,
    #[tree(branch)]
    pub root_module: Module,
}
