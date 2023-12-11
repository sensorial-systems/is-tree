use crate::knows_parent::KnowsParent;

pub trait HasParent<'a>: KnowsParent<'a> {
    fn parent(self) -> Self::Parent;
}
