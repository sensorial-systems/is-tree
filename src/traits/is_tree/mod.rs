pub mod tree_update;
pub mod has_branches;

pub use tree_update::*;
pub use has_branches::*;

use crate::TreeVisitor;

use super::has_identifier::HasPathSegment;

pub trait IsTree: HasPathSegment + TreeUpdate + HasBranches {
    fn iter(&self) -> TreeVisitor<'_, Self>
    where Self: Sized
    {
        TreeVisitor::new(self)
    }
}