use crate::{KnowsValue, HasValue, Visitor};

impl<'a, Parent, Value> KnowsValue<'a> for Visitor<Parent, Value> {
    type Value = Value;
}

impl<'a, Parent, Value> HasValue<'a> for Visitor<Parent, Value>
where Value: Clone
{
    fn value(&'a self) -> Self::Value {
        self.internal.value.clone()
    }
}
