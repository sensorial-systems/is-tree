use crate::{IsVisitor, RootVisitor, KnowsValue, HasValue};

impl<Value> KnowsValue for RootVisitor<Value> {
    type Value = Value;
}

impl<Value> HasValue for RootVisitor<Value> {
    fn value(self) -> Self::Value {
        self.value
    }
}

impl<Value> IsVisitor for RootVisitor<Value> {}
