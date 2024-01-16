mod has_root;
mod has_path_segment;
mod has_get;
mod has_relative_access;
mod has_parent;

use enum_as_inner::EnumAsInner;

use super::{LibraryVisitor, ModuleVisitor, ModuleParentVisitor};

#[derive(EnumAsInner)]
pub enum Visitors<'a> {
    Library(LibraryVisitor<'a>),
    Module(ModuleVisitor<'a>)
}

impl<'a> From<ModuleParentVisitor<'a>> for Visitors<'a> {
    fn from(visitor: ModuleParentVisitor<'a>) -> Self {
        match visitor {
            ModuleParentVisitor::Library(library) => Self::Library(library),
            ModuleParentVisitor::Module(module) => Self::Module(module)
        }
    }
}

// TODO: Remove this.
impl<'a> From<&LibraryVisitor<'a>> for Visitors<'a> {
    fn from(visitor: &LibraryVisitor<'a>) -> Self {
        Self::Library(visitor.clone())
    }
}

impl<'a> From<LibraryVisitor<'a>> for Visitors<'a> {
    fn from(visitor: LibraryVisitor<'a>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<ModuleVisitor<'a>> for Visitors<'a> {
    fn from(visitor: ModuleVisitor<'a>) -> Self {
        Self::Module(visitor)
    }
}
