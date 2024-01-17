use ::is_tree::*;
use super::{ModuleParentVisitor, LibraryVisitor};

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
