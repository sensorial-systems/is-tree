use crate::{KnowsValue, HasValue, RootVisitor};

impl<'a, Value> KnowsValue<'a> for RootVisitor<Value> {
    type Value = Value;
}

impl<'a, Value: Clone> HasValue<'a> for &'a RootVisitor<Value> {
    fn value(self) -> Self::Value {
        self.value.clone()
    }
}

impl<'a, Value: Clone> HasValue<'a> for &'a mut RootVisitor<Value> {
    fn value(self) -> Self::Value {
        self.value.clone()
    }
}
