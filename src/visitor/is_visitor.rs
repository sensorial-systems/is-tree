use crate::{IsVisitor, Visitor, KnowsParent, KnowsValue, HasValue, VisitorConstructor};

impl<'a, Parent, Value> KnowsParent<'a> for Visitor<Parent, Value> {
    type Parent = Parent;
}

impl<'a, Parent, Value> KnowsValue<'a> for Visitor<Parent, Value> {
    type Value = Value;
}

impl<'a, Parent, Value> HasValue<'a> for Visitor<Parent, Value>
where Value: Clone
{
    fn value(self) -> Self::Value {
        self.internal.value.clone()
    }
}

impl<'a, Parent, Value> KnowsValue<'a> for &'a Visitor<Parent, Value> {
    type Value = Value;
}

impl<'a, Parent, Value> HasValue<'a> for &'a Visitor<Parent, Value>
where Value: Clone
{
    fn value(self) -> Self::Value {
        self.internal.value.clone()
    }
}

impl<'a, Parent, Value> IsVisitor<'a> for Visitor<Parent, Value> {}

impl<'a, Parent, Value> VisitorConstructor<'a> for Visitor<Parent, Value> {
    type Owned = Visitor<Parent, Value>;
    fn new_with_parent(parent: Parent, value: Value) -> Self::Owned {
        Visitor::new_with_parent(parent, value)
    }
}

impl<'a, Parent, Value> IsVisitor<'a> for &'a Visitor<Parent, Value> {}

impl<'a, Parent, Value> VisitorConstructor<'a> for &'a Visitor<Parent, Value> {
    type Owned = Visitor<Parent, Value>;
    fn new_with_parent(_parent: Parent, _value: Value) -> Self::Owned {
        todo!()
        // Visitor::new_with_parent(parent, value)
    }
}