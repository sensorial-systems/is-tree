use ::is_tree::*;

use super::{super::LibraryVisitor, Module};

pub type ModuleVisitor<'a> = Visitor<ModuleParentVisitor<'a>, &'a Module>;

#[derive(Clone, IsTree)]
pub enum ModuleParentVisitor<'a> {
    Library(LibraryVisitor<'a>),
    Module(ModuleVisitor<'a>)
}

impl<'a> From<LibraryVisitor<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: LibraryVisitor<'a>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<ModuleVisitor<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: ModuleVisitor<'a>) -> Self {
        Self::Module(visitor)
    }
}