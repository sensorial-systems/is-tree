use crate::{Visitor, HasParent, KnowsParent};

impl<Parent, Value> KnowsParent for Visitor<Parent, Value> {
    type Parent = Parent;
}

impl<Parent, Value> HasParent for Visitor<Parent, Value>
where Parent: Clone
{
    fn parent(&self) -> Parent {
        self.internal.parent.clone()
    }
}
