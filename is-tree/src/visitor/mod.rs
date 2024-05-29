pub mod root_visitor;
pub use root_visitor::*;

mod has_path;
mod has_root;
mod has_parent;
mod has_value;
mod has_get;
mod knows_visitor;
mod has_visitor_constructor;
mod has_branches;
mod has_relative_access;

#[derive(Clone, Debug, Default)]
pub struct Visitor<Parent, Value> {
    pub parent: Parent,
    pub value: Value
}

impl<Parent, Value> Visitor<Parent, Value> {
    pub fn new(parent: Parent, value: Value) -> Self {
        Self { parent, value }
    }
}

impl<'a, Parent, Value> From<Visitor<Parent, &'a mut Value>> for Visitor<Parent, &'a Value> {
    fn from(visitor: Visitor<Parent, &'a mut Value>) -> Self {
        Self::new(visitor.parent, visitor.value)
    }
}

