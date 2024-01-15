use crate::{HasPathSegment, TreeVisitor, TreeUpdate, HasBranches};

pub trait IsTree<'a>: HasPathSegment + TreeUpdate<Self> + HasBranches<'a, Self> + Sized {
    fn iter(&self) -> TreeVisitor<&Self> {
        TreeVisitor::new(self)
    }
}