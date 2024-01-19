use is_tree::{KnowsGetType, HasGet, KnowsPathSegment, HasPathSegment};

use super::{Module, Library};


impl<'a> KnowsGetType for &'a Library {
    type GetType = &'a Module;
}

impl<'a> HasGet for &'a Library {
    fn get<PathSegment>(&self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        if &segment.into() == self.root_module.path_segment() {
            Some(&self.root_module)
        } else {
            None
        }
    }
}
