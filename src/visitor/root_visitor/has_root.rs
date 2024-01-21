use crate::{RootVisitor, KnowsRoot, HasRoot};

impl<'a, Value> KnowsRoot<'a> for RootVisitor<Value> {
    type Root = Self;
}

impl<'a, Value> HasRoot<'a> for RootVisitor<Value>
where Value: Clone
{
    fn root(&self) -> Self::Root {
        self.clone()
    }
}
