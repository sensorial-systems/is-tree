mod visitor;
use is_tree::IsTree;
pub use visitor::*;

mod knows_relative_access_type;
mod knows_visitor;
mod has_branches;
mod type_iterator;

#[derive(IsTree)]
pub struct Module {
    #[tree(path_segment)]
    pub name: String,
    pub children: Vec<Module>
}

impl From<String> for Module {
    fn from(name: String) -> Self {
        let children = Default::default();
        Self { name, children }
    }
}