mod visitor;
use is_tree::IsTree;
pub use visitor::*;

mod has_branches;

use super::{Module, Visitors};

#[derive(IsTree)]
#[tree(branches = "Module")]
#[tree(relative_visitor = "Visitors<&'a Library, &'a Module>")]
pub struct Library {
    #[tree(path_segment)]
    #[tree(type_iterator = "String")]
    pub name: String,
    #[tree(branch)]
    #[tree(type_iterator = "String")]
    pub root_module: Module,
}
