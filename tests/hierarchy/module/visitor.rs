use ::is_tree::*;

use crate::hierarchy::Library;

use super::{super::LibraryVisitor, Module};

pub type ModuleVisitor<'a> = Visitor<ModuleParentVisitor<'a>, &'a Module>;

#[derive(Clone, IsTree)]
pub enum ModuleParentVisitor<'a> {
    Library(LibraryVisitor<&'a Library>),
    Module(Box<ModuleVisitor<'a>>)
}

impl<'a> From<LibraryVisitor<&'a Library>> for ModuleParentVisitor<'a> {
    fn from(visitor: LibraryVisitor<&'a Library>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<ModuleVisitor<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: ModuleVisitor<'a>) -> Self {
        Self::Module(visitor.into())
    }
}
