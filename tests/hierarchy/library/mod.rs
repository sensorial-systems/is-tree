use is_tree::{TypeIter, TypeIterator, IterType};

pub mod visitor;
pub use visitor::*;

mod has_get;
mod has_path_segment;
mod knows_relative_access_type;
mod has_branches;

use super::{Module, Visitors};

pub struct Library {
    pub name: String,
    pub root_module: Module
}

impl<'a> From<&'a Library> for Visitors<'a> {
    fn from(value: &'a Library) -> Self {
        Self::Library(value.into())
    }
}

impl TypeIter<String> for Module {
    fn type_iterator(&self) -> TypeIterator<'_, String> {
        let mut strings = Vec::new();
        strings.push(&self.name);
        strings.extend(self.children.iter().flat_map(|m| m.iter_type::<String>()));
        strings.into()
    }
}

impl TypeIter<String> for Library {
    fn type_iterator(&self) -> TypeIterator<'_, String> {
        let mut strings = Vec::new();
        strings.push(&self.name);
        strings.extend(self.root_module.iter_type::<String>());
        strings.into()
    }
}
