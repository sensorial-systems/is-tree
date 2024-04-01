use crate::{HasPathSegment, HasBranches};

pub trait IsTree<'a>: HasPathSegment + HasBranches<'a> + Sized {
    // fn iter(&self) -> TreeVisitor<&Self> {
    //     TreeVisitor::new(self)
    // }
}