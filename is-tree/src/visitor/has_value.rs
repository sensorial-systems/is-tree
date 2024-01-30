use crate::{KnowsValue, HasValue, Visitor};

impl<'a, Parent, Value> KnowsValue<'a> for Visitor<Parent, Value> {
    type Value = Value;
}

impl<'a, Parent, Value> HasValue<'a> for Visitor<Parent, Value>
{
    fn value(self) -> Self::Value {
        self.value
    }
}

impl<'a, Parent, Value> HasValue<'a> for &'a Visitor<Parent, Value>
{
    fn value(self) -> Self::Value {
        &self.value
    }
}

impl<'a, Parent, Value> HasValue<'a> for &'a mut Visitor<Parent, Value>
{
    fn value(self) -> Self::Value {
        &mut self.value
    }
}
