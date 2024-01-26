use crate::{Visitor, HasParent, KnowsParent};

impl<'a, Parent, Value> KnowsParent<'a> for Visitor<Parent, Value> {
    type Parent = Parent;
}

impl<'a, Parent, Value> HasParent<'a> for &'a Visitor<Parent, Value>
where Parent: Clone
{
    fn parent(self) -> Parent {
        self.internal.parent.clone()
    }
}
