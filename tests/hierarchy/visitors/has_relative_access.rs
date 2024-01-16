use is_tree::{KnowsRelativeAccessType, HasRelativeAccess, KnowsPathSegment};

use super::Visitors;

impl<'a> KnowsRelativeAccessType for &'a Visitors<'a> {
    type RelativeType = Visitors<'a>;
}

impl<'a> HasRelativeAccess for &'a Visitors<'a> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
        where K: Into<<Self as KnowsPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.relative(path),
            Visitors::Module(module) => module.relative(path)
        }
    }
}
