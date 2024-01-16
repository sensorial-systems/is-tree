use is_tree::{KnowsGetType, HasGet, KnowsPathSegment, HasPathSegment};

use super::{Module, Library};


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
