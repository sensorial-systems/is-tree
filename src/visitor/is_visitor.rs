use crate::{IsVisitor, Visitor, KnowsParent, KnowsValue, HasValue, VisitorConstructor, KnowsVisitor};

impl<Parent, Value> KnowsParent for Visitor<Parent, Value> {
    type Parent = Parent;
}

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

impl<'a, Parent, Value> KnowsValue for &'a Visitor<Parent, Value> {
    type Value = Value;
}

impl<'a, Parent, Value> HasValue for &'a Visitor<Parent, Value>
where Value: Clone
{
    fn value(self) -> Self::Value {
        self.internal.value.clone()
    }
}

impl<Parent, Value> IsVisitor for Visitor<Parent, Value> {}

impl<Parent, Value> VisitorConstructor for Visitor<Parent, Value>
where Value: KnowsVisitor<Visitor = Visitor<Parent, Value>>
{
    fn new_with_parent(parent: Parent, value: Value) -> Visitor<Parent, Value> {
        Visitor::new_with_parent(parent, value)
    }
}

impl<'a, Parent, Value> IsVisitor for &'a Visitor<Parent, Value> {}

impl<'a, Parent, Value> VisitorConstructor for &'a Visitor<Parent, Value>
where Value: KnowsVisitor<Visitor = Visitor<Parent, Value>>,
{
    fn new_with_parent(parent: Parent, value: Value) -> Visitor<Parent, Value> {
        Visitor::new_with_parent(parent, value)
    }
}