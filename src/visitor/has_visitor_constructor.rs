use crate::{HasVisitorConstructor, Visitor, KnowsVisitor};

impl<Parent, Value> HasVisitorConstructor for Visitor<Parent, Value>
where Value: KnowsVisitor<Visitor = Visitor<Parent, Value>>
{
    fn new_with_parent(parent: Parent, value: Value) -> Visitor<Parent, Value> {
        Visitor::new_with_parent(parent, value)
    }
}
