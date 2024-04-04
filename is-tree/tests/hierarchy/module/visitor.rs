use ::is_tree::*;

use crate::hierarchy::Library;

use super::{super::LibraryVisitor, Module};

// TODO: Is it a duplicate logic of Visitors?
pub type ModuleVisitor<'a, Module> = Visitor<ModuleParentVisitor<'a>, Module>;

#[derive(Clone, IsTree)]
pub enum ModuleParentVisitor<'a> {
    Library(LibraryVisitor<&'a Library>),
    Module(Box<ModuleVisitor<'a, &'a Module>>)
}

impl<'a> From<LibraryVisitor<&'a Library>> for ModuleParentVisitor<'a> {
    fn from(visitor: LibraryVisitor<&'a Library>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<ModuleVisitor<'a, &'a Module>> for ModuleParentVisitor<'a> {
    fn from(visitor: ModuleVisitor<'a, &'a Module>) -> Self {
        Self::Module(visitor.into())
    }
}
