pub trait KnowsParent {
    type Parent;
}

pub trait HasParent: KnowsParent {
    fn parent(&self) -> Self::Parent;
}

// TODO: Remove duplicity
impl<T> KnowsParent for &T
where T: KnowsParent
{
    type Parent = T::Parent;
}