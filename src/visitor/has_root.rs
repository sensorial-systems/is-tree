use crate::{Visitor, KnowsRoot, HasRoot};

impl<'a, Parent, Value> KnowsRoot<'a> for Visitor<Parent, Value>
where &'a Parent: KnowsRoot<'a>, Parent: 'a
{
    type Root = <&'a Parent as KnowsRoot<'a>>::Root;
}

impl<'a, Parent, Value> HasRoot<'a> for &'a Visitor<Parent, Value>
where &'a Parent: HasRoot<'a>
{
    fn root(self) -> Self::Root {
        self.internal.parent.root()
    }
}
