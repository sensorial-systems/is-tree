use std::hash::Hash;

use crate::PathSegment;

pub trait IsPathSegment: PartialEq + Eq + Hash + Clone {
    fn root() -> Self;
    fn self_() -> Self;
    fn super_() -> Self;
    fn kind(&self) -> PathSegment<&Self> {
        if Self::root().eq(self) {
            PathSegment::Root
        } else if Self::self_().eq(self) {
            PathSegment::Self_
        } else if Self::super_().eq(self) {
            PathSegment::Super
        } else {
            PathSegment::Other(self)
        }
    }
}

impl IsPathSegment for String {
    fn root() -> Self {
        "root".to_string()
    }
    fn self_() -> Self {
        "self".to_string()
    }
    fn super_() -> Self {
        "super".to_string()
    }
}

impl IsPathSegment for () {
    fn root() -> Self {
        ()
    }
    fn self_() -> Self {
        ()
    }
    fn super_() -> Self {
        ()
    }
}
