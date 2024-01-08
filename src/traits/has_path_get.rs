// use crate::{HasBranches, HasPathSegment};

// pub trait HasPathGet<'a>: HasBranches<'a, Self>
// where Self: HasPathSegment + Sized
// {
//     fn path_get<K>(&self, path: impl IntoIterator<Item = K>) -> Option<&Self>
//     where K: Into<Self::PathSegment>
//     {
//         let mut path = path.into_iter();
//         if let Some(segment) = path.next() {
//             let segment = segment.into();
//             self
//                 .get(segment)
//                 .and_then(|branch|
//                     branch.path_get(path)
//                 )
//         } else {
//             Some(self)
//         }
//     }

//     fn path_get_mut<K>(&mut self, path: impl IntoIterator<Item = K>) -> Option<&mut Self>
//     where K: Into<Self::PathSegment> {
//         let mut path = path.into_iter();
//         if let Some(segment) = path.next() {
//             let segment = segment.into();
//             self
//                 .get_mut(segment)
//                 .and_then(|branch|
//                     branch.path_get_mut(path)
//                 )
//         } else {
//             Some(self)
//         }
//     }
// }

// impl<'a, T: HasBranches<'a, T>> HasPathGet<'a> for T {}