//! Tree iterator.

use crate::traits::has_branches::{HasBranches, HasBranchesAPI};
pub use crate::unsafe_::*;

/// An iterator over a tree.
pub struct TreeIterator<Visitor> {
    stack: Vec<Visitor>,
}

impl<'a, Visitor> TreeIterator<Visitor>
{
    /// Creates a new tree iterator.
    pub fn new<Value>(root: Value) -> Self
    where Value: Into<Visitor>,
          Visitor: UnsafeBorrow<'a> + UnsafeClone + 'a,
          Visitor::Borrow: HasBranches<Visitor>,
    {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build(root.into());
        iterator
    }

    fn build(&mut self, visitor: Visitor)
    where Visitor: UnsafeBorrow<'a> + UnsafeClone + 'a,
          Visitor::Borrow: HasBranches<Visitor>,
    {
        unsafe { self.stack.push(visitor.unsafe_clone()); }
        for child in unsafe { longer_ref(&visitor).borrow() }.branches_impl2::<Visitor>() {
            let visitor = child.into();
            self.build(visitor);
        }
    }
}

impl<Visitor> Iterator for TreeIterator<Visitor> {
    type Item = Visitor;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
