// use crate::{KnowsRelativeAccessType, RootVisitor, HasRelativeAccess, KnowsPathSegment, has_get::{HasGet, KnowsGetType}, KnowsParentVisitor, HasRoot, PathSegment, HasParent, IsPathSegment};

// impl<'a, Value> KnowsRelativeAccessType<'a> for &'a RootVisitor<Value>
// where Value: KnowsRelativeAccessType<'a>
// {
//     type RelativeType = Value::RelativeType;
// }

// impl<'a, Value> HasRelativeAccess<'a> for &'a RootVisitor<Value>
// where
//     Value: Copy + KnowsPathSegment + HasGet<'a>,
//       Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
//       Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
//     Value: KnowsRelativeAccessType<'a> + KnowsPathSegment + Clone + Copy + 'a,
//     Self: Into<Self::RelativeType> + HasRoot<'a>,

//     Self: HasGet<'a>,
//     <Self as KnowsGetType<'a>>::GetType:
//         KnowsParentVisitor<'a>
//         + Into<Self::RelativeType>
//         + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,
//     RootVisitor<Value>: Into<<<Self as KnowsGetType<'a>>::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,

//     &'a Self::RelativeType:
//         HasRelativeAccess<'a,
//             RelativeType = <Self as KnowsRelativeAccessType<'a>>::RelativeType,
//             PathSegment = <Self as KnowsPathSegment>::PathSegment
//         >
//         + HasParent<'a>
//         + HasRoot<'a>,
// {
//     fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
//         where K: Into<<Self as KnowsPathSegment>::PathSegment>,
//     {
//         let mut path = path.into_iter();
//         if let Some(segment) = path.next() {
//             let segment = segment.into();
//             match segment.kind() {
//                 PathSegment::Root | PathSegment::Self_ | PathSegment::Super => self.relative(path),
//                 PathSegment::Other(_segment) => 
//                     self
//                         .get(segment)
//                         .and_then(|value| {
//                             // FIXME: This is a hack.
//                             let visitor = value.into();
//                             let visitor = unsafe { std::mem::transmute::<_, &'a Self::RelativeType>(&visitor) };
//                             visitor.relative(path)
//                         }),
//             }
//         } else {
//             Some(self.into())
//         }
// }
// }
