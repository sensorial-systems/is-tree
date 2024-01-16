pub mod root_visitor;
use std::rc::Rc;

pub use root_visitor::*;

mod is_visitor;
mod has_path;
mod has_relative_access;
mod has_root;
mod has_get;
mod has_parent;
mod has_value;
mod knows_visitor;
mod has_visitor_constructor;

#[derive(Clone, Default)]
struct Internal<Parent, Value> {
    parent: Parent,
    value: Value
}

#[derive(Clone, Default)]
pub struct Visitor<Parent, Value> {
    internal: Rc<Internal<Parent, Value>>
}

impl<Parent, Value> Visitor<Parent, Value> {
    pub fn new_with_parent(parent: Parent, value: Value) -> Self {
        let internal = Rc::new(Internal { parent, value });
        Self { internal }
    }
}
