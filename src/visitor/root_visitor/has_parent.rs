use crate::{KnowsParent, RootVisitor, HasParent};

impl<'a, Value> KnowsParent<'a> for RootVisitor<Value> {
    type Parent = RootVisitor<Value>;
}

impl<'a, Value> KnowsParent<'a> for &'a RootVisitor<Value> {
    type Parent = RootVisitor<Value>;
}

impl<'a, Value> HasParent<'a> for &'a RootVisitor<Value>
where Value: Clone
{
    fn parent(self) -> Self::Parent {
        self.clone()
    }
}
