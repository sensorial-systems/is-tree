use crate::{HasPathSegment, IsPathSegment, PathSegment, UnsafeClone, UnsafeHasParent, UnsafeHasRoot};

use super::{HasBranches, HasGet, HasParent, HasRoot};

pub trait HasRelative<'a>: Sized {
    fn relative<K>(&'a self, path: impl IntoIterator<Item = K>) -> Option<Self>
    where K: Into<String>,
        Self: Clone + HasRoot + HasParent + HasPathSegment,
        &'a Self: HasBranches<Self>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            let visitor: Self = match segment.kind() {
                PathSegment::Self_ => self.clone(),
                PathSegment::Root => self.root(),
                PathSegment::Super => self.parent()?,
                PathSegment::Other(_) => self.get::<Self>(segment)?
            };
            unsafe { crate::unsafe_::longer_ref(&visitor) }.relative(path)
        } else {
            Some(self.clone())
        }
    }
}

pub trait UnsafeHasRelative<'a>: Sized {
    unsafe fn relative_mut<K>(&'a mut self, path: impl IntoIterator<Item = K>) -> Option<Self>
    where K: Into<String>,
        Self: UnsafeClone + UnsafeHasRoot + UnsafeHasParent + HasPathSegment,
        &'a mut Self: HasBranches<Self>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            let mut visitor: Self = match segment.kind() {
                PathSegment::Self_ => self.unsafe_clone(),
                PathSegment::Root => self.root_mut()?,
                PathSegment::Super => self.parent_mut()?,
                PathSegment::Other(_) => self.get::<Self>(segment)?
            };
            crate::unsafe_::longer_mut(&mut visitor).relative_mut(path)
        } else {
            Some(self.unsafe_clone())
        }
    }
}

impl<'a, T: Sized> HasRelative<'a> for T {}
impl<'a, T: Sized> UnsafeHasRelative<'a> for T {}