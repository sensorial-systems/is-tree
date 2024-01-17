use crate::{RootVisitor, HasRoot, KnowsRoot};

impl<Value> KnowsRoot for RootVisitor<Value> {
    type Root = RootVisitor<Value>;
}

impl<Value> HasRoot for RootVisitor<Value>
where Value: Clone
{
    fn root(&self) -> Self::Root {
        self.clone()
    }
}
