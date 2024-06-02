// This is required to build TreeIterator for a safe access order, i.e. leaves first, root last (needs clarification).
pub unsafe trait UnsafeClone {
    unsafe fn unsafe_clone(&self) -> Self;
}

// This is required to generalize TreeIterator with a single ::new method.
pub unsafe trait UnsafeBorrow<'a> {
    type Borrow;
    unsafe fn borrow(&'a self) -> Self::Borrow;
}

// TODO: Write about cases where this is needed.
#[inline]
pub unsafe fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
