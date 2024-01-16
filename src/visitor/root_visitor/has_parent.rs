use crate::{KnowsParent, RootVisitor, HasParent};

impl<Value> KnowsParent for RootVisitor<Value> {
    type Parent = RootVisitor<Value>;
}

impl<'a, Value> HasParent for &'a RootVisitor<Value>
where Value: Clone
{
    fn parent(self) -> Self::Parent {
        self.clone()
    }
}
