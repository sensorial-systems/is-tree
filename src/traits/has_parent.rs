
pub trait HasParent {
    type Parent;
    fn parent(&self) -> &Self::Parent;
}
