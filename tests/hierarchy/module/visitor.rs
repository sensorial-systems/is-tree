use ::is_tree::*;

use super::{super::LibraryVisitor, Module};

pub type ModuleVisitor<'a> = Visitor<ModuleParentVisitor<'a>, &'a Module>;

#[derive(Clone)]
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

impl<'a> HasPath<String> for ModuleParentVisitor<'a> {
    fn path(&self) -> Path<String> {
        match self {
            ModuleParentVisitor::Library(library) => library.path(),
            ModuleParentVisitor::Module(module) => module.path()
        }
    }
}

impl<'a> KnowsPathSegment for ModuleParentVisitor<'a> {
    type PathSegment = String;
}

impl<'a> HasPathSegment for ModuleParentVisitor<'a> {
    fn path_segment(&self) -> &Self::PathSegment {
        match self {
            ModuleParentVisitor::Library(library) => library.path_segment(),
            ModuleParentVisitor::Module(module) => module.path_segment()
        }
    }
}

impl<'a> KnowsRoot for ModuleParentVisitor<'a> {
    type Root = LibraryVisitor<'a>;
}

impl<'a> HasRoot for ModuleParentVisitor<'a> {
    fn root(&self) -> Self::Root {
        match self {
            ModuleParentVisitor::Library(library) => library.clone(),
            ModuleParentVisitor::Module(module) => module.parent().root()
        }
    }
}

impl<'a> HasRoot for &'a ModuleParentVisitor<'a> {
    fn root(&self) -> Self::Root {
        match self {
            ModuleParentVisitor::Library(library) => library.clone(),
            ModuleParentVisitor::Module(module) => module.clone().parent().root()
        }
    }
}

impl<'a> KnowsParent for ModuleParentVisitor<'a> {
    type Parent = ModuleParentVisitor<'a>;
}

impl<'a> HasParent for ModuleParentVisitor<'a> {
    fn parent(&self) -> ModuleParentVisitor<'a> {
        match self {
            ModuleParentVisitor::Library(library) => ModuleParentVisitor::Library(library.clone()),
            ModuleParentVisitor::Module(module) => module.clone().parent().clone()
        }
    }
}
