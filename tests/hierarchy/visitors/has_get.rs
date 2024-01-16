use is_tree::{KnowsGetType, HasGet, KnowsPathSegment};

use super::Visitors;

impl<'a> KnowsGetType for &Visitors<'a> {
    type GetType = Visitors<'a>;
}

impl<'a> HasGet for &Visitors<'a> {
    fn get<K>(self, key: K) -> Option<Self::GetType>
        where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.get(key).map(|value| value.into()),
            Visitors::Module(module) => module.get(key).map(|value| value.into())
        }
    }
}
