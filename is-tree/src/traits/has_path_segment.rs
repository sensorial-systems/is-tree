use crate::Path;

pub trait HasPath {
    fn path(&self) -> Path;
}

pub trait HasPathSegment {
    fn path_segment(&self) -> &String;

    fn is(&self, identifier: impl PartialEq<String>) -> bool {
        identifier.eq(self.path_segment())
    }

}

impl<T: HasPathSegment> HasPathSegment for &T {
    fn path_segment(&self) -> &String {
        (*self).path_segment()
    }
}

impl<T: HasPathSegment> HasPathSegment for &mut T {
    fn path_segment(&self) -> &String {
        (**self).path_segment()
    }
}
