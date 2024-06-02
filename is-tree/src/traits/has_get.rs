use crate::*;

pub trait HasGet<'a>: HasBranches<'a> + Sized
where Self::Branches: HasPathSegment
{
    fn get(self, segment: impl Into<String>) -> Option<Self::Branches>
    {
        let segment = segment.into();
        self.branches().find(|value| value.path_segment() == segment)
    }
}
