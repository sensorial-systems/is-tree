/// This is required to build TreeIterator for a safe access order, i.e. leaves first, root last. The Visitors are designed in a way where the parents are always constants and the visitor can only modify itself and its children. This design makes mutable relative access impossible.
// TODO: Why is constant parent necessary? Why is mutable relative access impossible?
// TODO: Move this explanation to the README.md file.
pub unsafe trait UnsafeClone {
    unsafe fn unsafe_clone(&self) -> Self;
}

/// This is required to generalize TreeIterator with a single ::new method.
pub unsafe trait UnsafeBorrow<'a> {
    type Borrow;
    unsafe fn borrow(&'a self) -> Self::Borrow;
}

/// TODO: Write about cases where this is needed.
#[inline]
pub unsafe fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }

#[inline]
pub unsafe fn longer_mut<'longer, T>(t: &mut T) -> &'longer mut T { unsafe { &mut *(t as *mut T) } }
