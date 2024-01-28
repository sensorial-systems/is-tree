use crate::{KnowsValue, HasValue, Visitor};

impl<'a, Parent, Value> KnowsValue<'a> for Visitor<Parent, Value> {
    type Value = Value;
}

impl<'a, Parent, Value> HasValue<'a> for &'a Visitor<Parent, Value>
{
    fn value(self) -> Self::Value {
        &self.internal.value
    }
}
