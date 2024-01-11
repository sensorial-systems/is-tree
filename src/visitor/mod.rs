pub mod root_visitor;
use std::rc::Rc;

pub use root_visitor::*;

mod is_visitor;
mod has_path;
mod has_relative_access;
mod has_root;
mod has_get;
mod has_parent;
mod knows_parent_visitor;

#[derive(Clone, Default)]
// FIXME: Make this private.
pub struct Internal<Parent, Value> {
    pub parent: Parent,
    pub value: Value
}

#[derive(Clone, Default)]
pub struct Visitor<Parent, Value> {
    // FIXME: Make this private.
    pub internal: Rc<Internal<Parent, Value>>
}

impl<Parent, Value> Visitor<Parent, Value> {
    pub fn new_with_parent(parent: Parent, value: Value) -> Self {
        let internal = Rc::new(Internal { parent, value });
        Self { internal }
    }
}
