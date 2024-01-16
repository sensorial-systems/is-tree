pub mod visitor;
pub use visitor::*;

use ::is_tree::*;
use super::{module::*, Visitors};

pub struct Library {
    pub name: String,
    pub root_module: Module
}

impl<'a> KnowsGetType for &'a Library {
    type GetType = &'a Module;
}

impl<'a> HasGet for &'a Library {
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        if &key.into() == self.root_module.path_segment() {
            Some(&self.root_module)
        } else {
            None
        }
    }
}

impl KnowsPathSegment for Library {
    type PathSegment = String;
}

impl HasPathSegment for Library {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl<'a> KnowsRelativeAccessType for &'a Library {
    type RelativeType = Visitors<'a>;
}

impl KnowsPathSegment for &Library {
    type PathSegment = String;
}

impl HasPathSegment for &Library {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

