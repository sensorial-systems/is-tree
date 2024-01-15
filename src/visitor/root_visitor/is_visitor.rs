use crate::{IsVisitor, RootVisitor, KnowsValue, HasValue};

impl<'a, Value> KnowsValue for RootVisitor<Value> {
    type Value = Value;
}

impl<'a, Value> HasValue for RootVisitor<Value> {
    fn value(self) -> Self::Value {
        self.value
    }
}

impl<'a, Value> IsVisitor for RootVisitor<Value> {}
