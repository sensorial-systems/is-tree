pub mod visitor;
pub use visitor::*;

mod has_get;
mod has_path_segment;
mod knows_relative_access_type;
mod knows_visitor;
mod has_branches;
mod type_iterator;

pub struct Module {
    pub name: String,
    pub children: Vec<Module>
}

impl From<String> for Module {
    fn from(name: String) -> Self {
        let children = Default::default();
        Self { name, children }
    }
}