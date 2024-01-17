use ::is_tree::*;
use super::ModuleParentVisitor;

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
