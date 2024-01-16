use is_tree::{KnowsRoot, HasRoot, HasParent};

use crate::hierarchy::LibraryVisitor;

use super::Visitors;

impl<'a> KnowsRoot for &'a Visitors<'a> {
    type Root = LibraryVisitor<'a>;
}

impl<'a> HasRoot for &'a Visitors<'a> {
    fn root(self) -> Self::Root {
        match self {
            Visitors::Library(library) => library.clone(),
            Visitors::Module(module) => module.clone().parent().root()
        }
    }
}
