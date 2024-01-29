mod has_root;
mod has_parent;
mod has_value;
mod has_path;
mod has_get;
mod has_branches;
mod has_relative_access;

#[derive(Clone, Copy, Default)]
pub struct RootVisitor<Value> {
    pub value: Value
}

impl<Value> RootVisitor<Value> {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl<'a, T> From<RootVisitor<&'a mut T>> for RootVisitor<&'a T> {
    fn from(visitor: RootVisitor<&'a mut T>) -> Self {
        Self::new(visitor.value)
    }
}

impl<T> From<T> for RootVisitor<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
