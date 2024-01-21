use crate::{KnowsValue, HasValue, Visitor};

impl<Parent, Value> KnowsValue<'_> for Visitor<Parent, Value> {
    type Value = Value;
}

impl<Parent, Value> HasValue<'_> for Visitor<Parent, Value>
where Value: Clone
{
    fn value(&self) -> Self::Value {
        self.internal.value.clone()
    }
}
