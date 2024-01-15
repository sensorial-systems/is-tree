pub trait KnowsParent {
    type Parent;
}

pub trait HasParent: KnowsParent {
    fn parent(self) -> Self::Parent;
}
