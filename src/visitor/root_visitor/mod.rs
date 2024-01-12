mod is_visitor;
mod has_get;
mod has_root;
mod has_parent;
mod has_path;
mod has_relative_access;

#[derive(Clone, Copy, Default)]
pub struct RootVisitor<Value> {
    pub value: Value
}

impl<'a, Value> RootVisitor<Value> {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
