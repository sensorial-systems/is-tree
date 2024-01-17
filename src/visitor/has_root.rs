use crate::{KnowsRoot, Visitor, HasRoot};

impl<Parent, Value> KnowsRoot for Visitor<Parent, Value>
where Parent: KnowsRoot
{
    type Root = Parent::Root;
}

impl<'a, Parent, Value> HasRoot for Visitor<Parent, Value>
where Parent: HasRoot
{
    fn root(&self) -> Self::Root {
        self.internal.parent.root()
    }
}
