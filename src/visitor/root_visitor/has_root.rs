use crate::{RootVisitor, HasRoot, KnowsRoot};

impl<Value> KnowsRoot for RootVisitor<Value> {
    type Root = RootVisitor<Value>;
}

impl<'a, Value> KnowsRoot for &'a RootVisitor<Value> {
    type Root = RootVisitor<Value>;
}

impl<'a, Value> HasRoot for &'a RootVisitor<Value>
where Value: Clone
{
    fn root(self) -> Self::Root {
        self.clone()
    }
}
