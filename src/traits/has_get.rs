use crate::*;

pub trait KnowsGetType<'a> {
    type GetType;
}

pub trait HasGet<'a>: KnowsGetType<'a>
where Self::GetType: KnowsPathSegment
{
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment>;
    // TODO: Add default implementation with PathSegment search on HasBranches iterator.
}

impl<'a, T: KnowsGetType<'a>> KnowsGetType<'a> for &'a T {
    type GetType = <T as KnowsGetType<'a>>::GetType;
}

// impl<'a, T: HasGet<'a>> HasGet<'a> for &'a T
// where Self::GetType: KnowsPathSegment<PathSegment = <T::GetType as KnowsPathSegment>::PathSegment>
// {
//     fn get<PathSegment>(&'a self, segment: PathSegment) -> Option<Self::GetType>
//     where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment> 
//     {
//         // todo!()
//         // (*self).get(segment)
//     }
// }
