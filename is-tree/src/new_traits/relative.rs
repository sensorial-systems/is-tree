use crate::{HasPathSegment, IsPathSegment, PathSegment};

use super::{HasBranches, HasGet, HasParent, HasRoot};

pub trait HasRelative<'a>: Sized {
    fn relative<K>(&'a self, path: impl IntoIterator<Item = K>) -> Option<Self>
    where K: Into<String>,
        Self: Clone + HasRoot + HasParent + HasPathSegment,
        &'a Self: HasBranches<Self>
    {
        #[inline]
        fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            let visitor: Self = match segment.kind() {
                PathSegment::Self_ => self.clone(),
                PathSegment::Root => self.root(),
                PathSegment::Super => self.parent()?,
                PathSegment::Other(_) => self.get::<Self>(segment)?
            };
            longer_ref(&visitor).relative(path)
        } else {
            Some(self.clone())
        }
    }
}

impl<'a, T: Sized> HasRelative<'a> for T {}