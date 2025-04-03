//! Unsafe traits and functions cautiously used in the library but made unsafe to discourage their external use.

pub unsafe trait UnsafeFrom<T> {
    /// Converts the object from another object.
    unsafe fn unsafe_from(t: T) -> Self;

}

/// This is required to generalize TreeIterator with a single ::new method.
pub unsafe trait UnsafeClone {
    /// Clones the object.
    unsafe fn unsafe_clone(&self) -> Self;
}

/// This is required to generalize TreeIterator with a single ::new method.
pub unsafe trait UnsafeBorrow<'a> {
    /// The type of the borrowed object.
    type Borrow;
    /// Borrows the object.
    unsafe fn borrow(&'a self) -> Self::Borrow;
}

/// Makes the reference live longer.
#[inline]
pub unsafe fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }

/// Makes the mutable reference live longer.
#[inline]
pub unsafe fn longer_mut<'longer, T>(t: &mut T) -> &'longer mut T { unsafe { &mut *(t as *mut T) } }
