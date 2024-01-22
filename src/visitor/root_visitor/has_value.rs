use crate::{KnowsValue, HasValue, RootVisitor};

impl<'a, Value> KnowsValue<'a> for RootVisitor<Value> {
    type Value = Value;
}

impl<'a, Value: Clone> HasValue<'a> for RootVisitor<Value> {
    fn value(&'a self) -> Self::Value {
        self.value.clone()
    }
}
