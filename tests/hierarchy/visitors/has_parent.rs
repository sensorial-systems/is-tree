use is_tree::{KnowsParent, HasParent};

use super::Visitors;

impl<'a> KnowsParent for &Visitors<'a> {
    type Parent = Visitors<'a>;
}

impl<'a> HasParent for &Visitors<'a> {
    fn parent(self) -> Visitors<'a> {
        match self {
            Visitors::Library(visitor) => (*visitor).into(),
            Visitors::Module(visitor) => visitor.parent().into()
        }
    }
}
