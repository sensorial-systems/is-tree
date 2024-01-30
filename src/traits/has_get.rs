use crate::*;

pub trait HasGet<'a>: HasBranches<'a>
where Self::Branches: KnowsPathSegment
{
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::Branches>
    where PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment>;
    // TODO: Add default implementation with PathSegment search on HasBranches iterator.
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
