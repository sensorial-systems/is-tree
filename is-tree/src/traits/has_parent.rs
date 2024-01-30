pub trait KnowsParent<'a> {
    type Parent;
}

pub trait HasParent<'a>: KnowsParent<'a> {
    fn parent(self) -> Self::Parent;
}

impl<'a, T: KnowsParent<'a>> KnowsParent<'a> for &'a T
{
    type Parent = T::Parent;
}

impl<'a, T: KnowsParent<'a>> KnowsParent<'a> for &'a mut T
{
    type Parent = T::Parent;
}
