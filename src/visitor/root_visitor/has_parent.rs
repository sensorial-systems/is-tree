use crate::{KnowsParent, RootVisitor, HasParent};

impl<Value> KnowsParent for RootVisitor<Value> {
    type Parent = RootVisitor<Value>;
}

impl<Value> HasParent for RootVisitor<Value>
where Value: Clone
{
    fn parent(&self) -> Self::Parent {
        self.clone()
    }
}
