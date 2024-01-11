use crate::{IsVisitor, RootVisitor, KnowsValue, HasValue};

impl<'a, Value> KnowsValue<'a> for RootVisitor<Value> {
    type Value = Value;
}

impl<'a, Value> HasValue<'a> for RootVisitor<Value> {
    fn value(self) -> Self::Value {
        self.value
    }
}

impl<'a, Value> IsVisitor<'a> for RootVisitor<Value> {}
