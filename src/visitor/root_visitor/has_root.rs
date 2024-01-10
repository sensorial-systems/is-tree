use crate::{RootVisitor, HasRoot, KnowsRoot};

impl<'a, Value> KnowsRoot<'a> for &'a RootVisitor<Value> {
    type Root = RootVisitor<Value>;
}

impl<'a, Value> HasRoot<'a> for &'a RootVisitor<Value>
where Value: Clone
{
    fn root(self) -> Self::Root {
        self.clone()
    }
}
