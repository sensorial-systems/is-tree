use crate::*;

pub trait HasGet<'a>: HasBranches<'a> + Sized
where Self::Branches: HasPathSegment
{
    fn get<PathSegment>(self, segment: PathSegment) -> Option<Self::Branches>
    where PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment> {
        let segment = segment.into();
        self.branches().find(|value| value.path_segment() == &segment)
    }
}
