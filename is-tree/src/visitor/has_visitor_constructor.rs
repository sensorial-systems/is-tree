use crate::{HasVisitorConstructor, Visitor, KnowsVisitor};

impl<'a, Parent, Value> HasVisitorConstructor<'a> for Visitor<Parent, Value>
where Value: KnowsVisitor<'a, Visitor = Visitor<Parent, Value>>
{
    fn new(parent: Parent, value: Value) -> Visitor<Parent, Value> {
        Visitor::new(parent, value)
    }
}
