use crate::*;

pub struct TreeIterator<Visitor> {
    stack: Vec<Visitor>,
}

impl<'a, Visitor> TreeIterator<Visitor>
where
    Visitor: Clone + HasBranches<'a> + 'a,
    <Visitor as KnowsBranches<'a>>::Branches: Into<Visitor>,
{
    pub fn new<Value>(root: &'a Value) -> Self
    where &'a Value: Into<Visitor> 
    {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build(root.into());
        iterator
    }

    fn build(&mut self, visitor: Visitor) {
        self.stack.push(visitor.clone());
        // FIXME: This is a hack to get around the borrow checker.
        let visitor = unsafe { &*(&visitor as *const Visitor) };
        for child in visitor.branches() {
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
