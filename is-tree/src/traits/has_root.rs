pub trait HasRoot {
    fn root(&self) -> Self;
}

// By design, accessing a Visitor parent is unsafe.
pub unsafe trait UnsafeHasRoot: Sized {
    unsafe fn root_mut(&mut self) -> Self;
}