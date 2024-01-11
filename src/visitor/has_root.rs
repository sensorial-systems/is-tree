use crate::{KnowsRoot, Visitor, HasRoot};

impl<'a, Parent, Value> KnowsRoot<'a> for Visitor<Parent, Value>
where Parent: KnowsRoot<'a>
{
    type Root = Parent::Root;
}

impl<'a, Parent, Value> KnowsRoot<'a> for &'a Visitor<Parent, Value>
where Parent: KnowsRoot<'a>
{
    type Root = Parent::Root;
}

impl<'a, Parent, Value> HasRoot<'a> for &'a Visitor<Parent, Value>
where Parent: HasRoot<'a> + Clone
{
    fn root(self) -> Self::Root {
        self.internal.parent.clone().root()
    }
}
