use crate::{KnowsValue, RootVisitor, HasValue};

impl<Value> KnowsValue for RootVisitor<Value> {
    type Value = Value;
}

impl<Value: Clone> HasValue for RootVisitor<Value> {
    fn value(&self) -> Self::Value {
        self.value.clone()
    }
}
