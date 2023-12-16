use crate::{HasPathSegment, PathSegment, HasRoot, KnowsParent, HasParent};
use crate::traits::*;

pub trait HasRelativeAccessType<'a> {
    type RelativeType;
}

pub trait HasRelativeAccess<'a>:
      HasRelativeAccessType<'a>
    + HasPathSegment
    + Into<Self::RelativeType>
    + HasParent<'a>
    + HasRoot<'a>
    where
    Self::Parent: Into<Self::RelativeType>,
    Self::Root: Into<Self::RelativeType>,
    
    Self::RelativeType:
      HasRelativeAccess<'a>
    + HasRelativeAccessType<'a, RelativeType = Self::RelativeType>
    + HasPathSegment<PathSegment = Self::PathSegment>
    + HasParent<'a>
    + HasRoot<'a, Root = Self::Root>,
    <Self::RelativeType as KnowsParent<'a>>::Parent: Into<Self::RelativeType>
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as HasPathSegment>::PathSegment>,
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            match segment.kind() {
                PathSegment::Root => Some(self.root().into()),
                PathSegment::Self_ => self.relative(path),
                PathSegment::Super => self.parent().into().relative(path),
                _ => todo!("Not implemented yet")
                // Identifier::Super => self
                //     .parent
                //     .as_ref()
                //     .and_then(|parent| parent.relative(path)),
                // Identifier::Other(segment) => self
                //     .value
                //     .get(segment.clone())
                //     .and_then(|branch|
                //         self.child(branch)
                //             .relative(path)
                //     )
            }
        } else {
            Some(self.into())
        }
    }
}
