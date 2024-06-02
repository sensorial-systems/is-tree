pub trait HasParent: Sized {
    fn parent(&self) -> Option<Self>;
}

// By design, accessing a Visitor parent is unsafe.
pub unsafe trait UnsafeHasParent: Sized {
    unsafe fn parent_mut(&mut self) -> Option<Self>;
}