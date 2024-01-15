pub struct TreeVisitor<Visitor> {
    stack: Vec<Visitor>,
}

impl<Visitor> TreeVisitor<Visitor>
where Visitor: Clone
{
    pub fn new<Value: Into<Visitor>>(root: Value) -> Self {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build(root);
        iterator
    }

    fn build<Value: Into<Visitor>>(&mut self, visitor: Value) {
        let visitor = visitor.into();
        self.stack.push(visitor.clone());
        // for child in visitor.branches() {
            // let visitor = visitor.child(child);
            // self.build(visitor);
        // }
    }
}

impl<Visitor> Iterator for TreeVisitor<Visitor> {
    type Item = Visitor;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
