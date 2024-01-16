pub mod visitor;
pub use visitor::*;

mod has_get;
mod has_path_segment;
mod knows_relative_access_type;
mod knows_visitor;

pub struct Module {
    pub name: String,
    pub children: Vec<Module>
}
