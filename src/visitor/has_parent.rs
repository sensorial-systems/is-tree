use crate::{Visitor, HasParent, KnowsParent};

impl<Parent, Value> KnowsParent for Visitor<Parent, Value> {
    type Parent = Parent;
}

impl<'a, Parent, Value> HasParent for &'a Visitor<Parent, Value>
where Parent: Clone
{
    fn parent(self) -> Parent {
        self.internal.parent.clone()
    }
}
