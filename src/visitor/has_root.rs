use crate::{Visitor, KnowsRoot, HasRoot};

impl<'a, Parent, Value> KnowsRoot<'a> for Visitor<Parent, Value>
where Parent: KnowsRoot<'a>
{
    type Root = Parent::Root;
}

impl<'a, Parent, Value> HasRoot<'a> for Visitor<Parent, Value>
where Parent: HasRoot<'a>
{
    fn root(&self) -> Self::Root {
        self.internal.parent.root()
    }
}
