use is_tree::{HasGet, KnowsPathSegment, HasPathSegment};

use super::Module;

impl<'a> HasGet<'a> for &'a Module {
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::Branches>
        where PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment>
    {
        let key = segment.into();
        self.children.iter().find(|child| &key == child.path_segment())
    }
}
