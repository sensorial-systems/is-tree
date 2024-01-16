use is_tree::{KnowsParent, HasParent};

use super::Visitors;

impl<'a> KnowsParent for &'a Visitors<'a> {
    type Parent = Visitors<'a>;
}

impl<'a> HasParent for &'a Visitors<'a> {
    fn parent(self) -> Visitors<'a> {
        match self {
            Visitors::Library(visitor) => visitor.into(),
            Visitors::Module(visitor) => visitor.parent().into()
        }
    }
}
