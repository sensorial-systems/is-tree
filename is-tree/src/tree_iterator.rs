use crate::*;
pub use crate::unsafe_::*;

pub struct TreeIterator<Visitor> {
    stack: Vec<Visitor>,
}

impl<'a, Visitor> TreeIterator<Visitor>
{
    pub fn new<Value>(root: Value) -> Self
    where Value: Into<Visitor>,
          Visitor: UnsafeBorrow<'a> + UnsafeClone + 'a,
          Visitor::Borrow: HasBranches<Visitor>,
    {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.constant_build(root.into());
        iterator
    }

    fn constant_build(&mut self, visitor: Visitor)
    where Visitor: UnsafeBorrow<'a> + UnsafeClone + 'a,
          Visitor::Borrow: HasBranches<Visitor>,
    {
        unsafe { self.stack.push(visitor.unsafe_clone()); }
        for child in unsafe { longer_ref(&visitor).borrow() }.branches::<Visitor>() {
            let visitor = child.into();
            self.constant_build(visitor);
        }
    }
}

impl<Visitor> Iterator for TreeIterator<Visitor> {
    type Item = Visitor;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
