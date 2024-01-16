use crate::{KnowsValue, Visitor, HasValue};

impl<Parent, Value> KnowsValue for Visitor<Parent, Value> {
    type Value = Value;
}

impl<Parent, Value> HasValue for Visitor<Parent, Value>
where Value: Clone
{
    fn value(self) -> Self::Value {
        self.internal.value.clone()
    }
}
