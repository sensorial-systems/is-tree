mod has_root;
mod has_parent;
mod has_value;
mod has_path;
mod has_get;
mod has_branches;

#[derive(Clone, Copy, Default)]
pub struct RootVisitor<Value> {
    pub value: Value
}

impl<Value> RootVisitor<Value> {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
