use is_tree::{KnowsGetType, HasGet, KnowsPathSegment, HasPathSegment};

use super::Module;

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
