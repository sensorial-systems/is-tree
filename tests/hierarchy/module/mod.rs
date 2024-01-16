pub mod visitor;
pub use visitor::*;

use ::is_tree::*;
use super::Visitors;

pub struct Module {
    pub name: String,
    pub children: Vec<Module>
}

impl<'a> KnowsRelativeAccessType for &'a Module {
    type RelativeType = Visitors<'a>;
}

impl<'a> KnowsGetType for &'a Module {
    type GetType = &'a Module;
}

impl<'a> HasGet for &'a Module {
    fn get<K>(self, key: K) -> Option<Self::GetType>
        where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        let key = key.into();
        self.children.iter().find(|child| &key == child.path_segment())
    }
}

impl KnowsPathSegment for Module {
    type PathSegment = String;
}

impl HasPathSegment for Module {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl KnowsPathSegment for &Module {
    type PathSegment = String;
}

impl<'a> KnowsVisitor for &'a Module {
    type Visitor = ModuleVisitor<'a>;
}

impl HasPathSegment for &Module {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}
