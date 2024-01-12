// use crate::{KnowsRelativeAccessType, Visitor, PathSegment, KnowsPathSegment, HasParent, HasRelativeAccess, KnowsParent, has_get::{KnowsGetType, HasGet}, KnowsParentVisitor, KnowsRoot, HasRoot, IsPathSegment};

// impl<'a, Parent, Value> KnowsRelativeAccessType<'a> for Visitor<Parent, Value>
// where Value: KnowsRelativeAccessType<'a>
// {
//     type RelativeType = Value::RelativeType;
// }

// impl<'a, Parent, Value> KnowsRelativeAccessType<'a> for &'a Visitor<Parent, Value>
// where Value: KnowsRelativeAccessType<'a>
// {
//     type RelativeType = Value::RelativeType;
// }

// impl<'a, Parent, Value> HasRelativeAccess<'a> for &'a Visitor<Parent, Value>
// where
//     Self: Into<Self::RelativeType> + KnowsPathSegment,
//     Parent: Into<Self::RelativeType> + Clone + 'a,
//     Value: KnowsPathSegment + KnowsRelativeAccessType<'a> + KnowsParentVisitor<'a, ParentVisitor = Parent> + 'a,

//     Self: HasRoot<'a>,
//     <Self as KnowsRoot<'a>>::Root: Into<Self::RelativeType>,
//     &'a Parent: HasRoot<'a, Root = <Self as KnowsRoot<'a>>::Root>,
//     &'a Value::RelativeType: HasRoot<'a, Root = <Self as KnowsRoot<'a>>::Root>,

//     Self: HasGet<'a>,
//     <Self as KnowsGetType<'a>>::GetType:
//         KnowsParentVisitor<'a>
//         + Into<Self::RelativeType>
//         + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,
//     Self: Into<<<Self as KnowsGetType<'a>>::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,

//     <Self as KnowsParent<'a>>::Parent: Into<Self::RelativeType>,
//     &'a Value::RelativeType:
//       HasRelativeAccess<'a>
//     + KnowsRelativeAccessType<'a, RelativeType = Self::RelativeType>
//     + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>
//     + HasParent<'a>,
// {
//     fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
//     where K: Into<<Self as KnowsPathSegment>::PathSegment>
//     {
//             let mut path = path.into_iter();
//             if let Some(segment) = path.next() {
//                 let segment = segment.into();
//                 let visitor = match segment.kind() {
//                     PathSegment::Self_ => self.into(),
//                     PathSegment::Root => self.root().into(),
//                     PathSegment::Super => self.parent().into(),
//                     PathSegment::Other(_) => self.get(segment)?.into()
//                 };
//                 // FIXME: This is a hack.
//                 let visitor = unsafe { std::mem::transmute::<_, &'a Value::RelativeType>(&visitor) };
//                 visitor.relative(path)
//             } else {
//                 Some(self.into())
//             }    
//     }
// }
