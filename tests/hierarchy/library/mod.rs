use is_tree::{TypeIter, TypeIterator, IterType, Visitor, HasVisitor};

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

impl<'a> TypeIter<'a, Visitor<Visitors<'a>, &'a String>> for Module {
    fn type_iterator(&'a self, parent: Option<Visitors<'a>>) -> TypeIterator<Visitor<Visitors<'a>, &'a String>> {
        let mut collection = Vec::new();
        let parent = parent.unwrap();
        let visitor = Visitor::new(parent.clone(), &self.name);
        collection.push(visitor.clone());
        collection.extend(self.children.iter().flat_map(|child| child.iter_type_with_parent::<String>(Some(parent.clone()))));
        collection.into()
    }
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a>, &'a String>> for Library {
    fn type_iterator(&'a self, _parent: Option<Visitors<'a>>) -> TypeIterator<Visitor<Visitors<'a>, &'a String>> {
        let mut collection = Vec::new();
        let visitor = Visitor::new(self.visitor().into(), &self.name);
        collection.push(visitor.clone());
        collection.extend(self.root_module.iter_type_with_parent::<String>(Some(self.visitor().into())));
        collection.into()
    }
}
