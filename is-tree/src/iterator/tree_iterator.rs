use crate::*;

pub struct TreeIterator<Visitor> {
    stack: Vec<Visitor>,
}

impl<'a, Visitor> TreeIterator<Visitor>
where
    Visitor: HasBranches<'a> + Clone,
    Visitor::Branches: Into<Visitor>
{
    pub fn new<Value>(root: Value) -> Self
    where Value: Into<Visitor> 
    {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build(&root.into());
        iterator
    }

    fn build(&mut self, visitor: &Visitor)
    {
        self.stack.push(visitor.clone());
        for child in visitor.clone().branches() {
            let visitor = child.into();
            self.build(&visitor);
        }
    }
}

impl<Visitor> Iterator for TreeIterator<Visitor> {
    type Item = Visitor;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
