use is_tree::{KnowsGetType, HasGet, KnowsPathSegment, HasPathSegment};

use super::Module;

impl<'a> KnowsGetType for &'a Module {
    type GetType = &'a Module;
}

impl<'a> HasGet for &'a Module {
    fn get<PathSegment>(&self, segment: PathSegment) -> Option<Self::GetType>
        where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        let key = segment.into();
        self.children.iter().find(|child| &key == child.path_segment())
    }
}
