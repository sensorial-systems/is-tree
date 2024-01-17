use is_tree::{KnowsRelativeAccessType, HasRelativeAccess, KnowsPathSegment};

use super::Visitors;

impl<'a> KnowsRelativeAccessType for Visitors<'a> {
    type RelativeType = Visitors<'a>;
}

impl<'a> HasRelativeAccess for Visitors<'a> {
    fn relative<K>(&self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
        where K: Into<<Self as KnowsPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.relative(path),
            Visitors::Module(module) => module.relative(path)
        }
    }
}
