mod visitor;
pub use visitor::*;

mod has_get;
mod has_path_segment;
mod knows_relative_access_type;
mod has_branches;
mod type_iterator;

use super::{Module, Visitors};

pub struct Library {
    pub name: String,
    pub root_module: Module
}
