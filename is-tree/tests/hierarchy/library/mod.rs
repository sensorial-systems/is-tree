mod visitor;
use is_tree::IsTree;
pub use visitor::*;

mod knows_relative_access_type;
mod has_branches;
mod type_iterator;

use super::{Module, Visitors};

#[derive(IsTree)]
#[tree(branches = "Module")]
pub struct Library {
    #[tree(path_segment)]
    pub name: String,
    pub root_module: Module
}
