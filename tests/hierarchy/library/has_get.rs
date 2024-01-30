use is_tree::{HasGet, KnowsPathSegment, HasPathSegment};

use super::Library;

impl<'a> HasGet<'a> for &'a Library {
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::Branches>
    where PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment>
    {
        if &segment.into() == self.root_module.path_segment() {
            Some(&self.root_module)
        } else {
            None
        }
    }
}
