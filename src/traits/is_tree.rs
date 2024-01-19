use crate::{HasPathSegment, TreeUpdate, HasBranches};

pub trait IsTree<'a>: HasPathSegment + TreeUpdate<Self> + HasBranches + Sized {
    // fn iter(&self) -> TreeVisitor<&Self> {
    //     TreeVisitor::new(self)
    // }
}