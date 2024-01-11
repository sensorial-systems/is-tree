use crate::{KnowsParentVisitor, Visitor};

impl<'a, Parent, Value> KnowsParentVisitor<'a> for Visitor<Parent, Value>
where Value: KnowsParentVisitor<'a>,
{
    type ParentVisitor = Value::ParentVisitor;
}
