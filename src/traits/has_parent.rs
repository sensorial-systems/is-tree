pub trait KnowsParent<'a> {
    type Parent;
}

pub trait HasParent<'a>: KnowsParent<'a> {
    fn parent(self) -> Self::Parent;
}
