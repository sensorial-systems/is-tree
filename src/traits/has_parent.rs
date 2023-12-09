use crate::knows_parent::KnowsParent;

pub trait HasParent: KnowsParent {
    fn parent(&self) -> &Self::Parent;
}
