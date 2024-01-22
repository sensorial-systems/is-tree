use is_tree::{KnowsGetType, HasGet, KnowsPathSegment, HasPathSegment};

use super::{Module, Library};

impl<'a> KnowsGetType<'a> for Library {
    type GetType = &'a Module;
}

impl<'a> HasGet<'a> for Library {
    fn get<PathSegment>(&'a self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        if &segment.into() == self.root_module.path_segment() {
            Some(&self.root_module)
        } else {
            None
        }
    }
}

impl<'a> HasGet<'a> for &'a Library {
    fn get<PathSegment>(&'a self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        if &segment.into() == self.root_module.path_segment() {
            Some(&self.root_module)
        } else {
            None
        }
    }
}
