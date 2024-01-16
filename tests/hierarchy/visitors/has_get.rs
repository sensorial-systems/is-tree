use is_tree::{KnowsGetType, HasGet, KnowsPathSegment};

use super::Visitors;

impl<'a> KnowsGetType for &'a Visitors<'a> {
    type GetType = Visitors<'a>;
}

impl<'a> HasGet for &'a Visitors<'a> {
    fn get<K>(self, key: K) -> Option<Self::GetType>
        where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.get(key).map(|value| value.into()),
            Visitors::Module(module) => module.get(key).map(|value| value.into())
        }
    }
}
