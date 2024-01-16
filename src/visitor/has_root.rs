use crate::{KnowsRoot, Visitor, HasRoot};

impl<Parent, Value> KnowsRoot for Visitor<Parent, Value>
where Parent: KnowsRoot
{
    type Root = Parent::Root;
}

impl<'a, Parent, Value> HasRoot for &'a Visitor<Parent, Value>
where Parent: HasRoot + Clone
{
    fn root(self) -> Self::Root {
        self.internal.parent.clone().root()
    }
}
