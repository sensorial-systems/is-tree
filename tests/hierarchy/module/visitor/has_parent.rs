use ::is_tree::*;
use super::ModuleParentVisitor;

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
