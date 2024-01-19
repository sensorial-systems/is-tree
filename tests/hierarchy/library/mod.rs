pub mod visitor;
use is_tree::{HasBranches, KnowsBranches};
pub use visitor::*;

mod has_get;
mod has_path_segment;
mod knows_relative_access_type;

use super::Module;

pub struct Library {
    pub name: String,
    pub root_module: Module
}

impl<'a> KnowsBranches for &'a Library {
    type Branches = &'a Module;
}

impl<'a> HasBranches for &'a Library {
    fn branches(&self) -> impl Iterator<Item = Self::Branches> {
        std::iter::once(&self.root_module)
    }
}