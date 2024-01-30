use crate::{HasVisitorConstructor, Visitor, KnowsVisitor};

impl<'a, Parent, Value> HasVisitorConstructor<'a> for Visitor<Parent, Value>
where Value: KnowsVisitor<'a, Visitor = Visitor<Parent, Value>>
{
    // TODO: Rename new_with_parent?
    fn new_with_parent(parent: Parent, value: Value) -> Visitor<Parent, Value> {
        Visitor::new(parent, value)
    }
}
