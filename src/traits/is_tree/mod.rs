pub mod tree_update;
pub mod has_branches;

pub use tree_update::*;
pub use has_branches::*;

use crate::{HasPathSegment, TreeVisitor};

pub trait IsTree<'a>: HasPathSegment + TreeUpdate<Self> + HasBranches<'a, Self> + Sized{
    fn iter(&self) -> TreeVisitor<&Self>
    where Self: Sized
    {
        TreeVisitor::new(self)
    }
}