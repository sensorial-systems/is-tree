use crate::*;

pub struct TreeIterator<'a, Value>
where Value: KnowsRelativeAccessType<'a>
{
    stack: Vec<Value::RelativeType>,
}

impl<'a, Value> TreeIterator<'a, Value>
where
    Value: KnowsRelativeAccessType<'a>,
    Value: Into<Value::RelativeType>,
    Value::RelativeType: HasBranches<'a> + Clone,
    <Value::RelativeType as KnowsBranches<'a>>::Branches: Into<Value::RelativeType>
{
    pub fn new(root: Value) -> Self
    {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.build( root.into());
        iterator
    }

    fn build(&mut self, visitor: Value::RelativeType) {
        self.stack.push(visitor.clone());
        for child in visitor.clone().branches() {
            self.build( child.into());
        }
    }
}

impl<'a, Value> Iterator for TreeIterator<'a, Value>
where Value: KnowsRelativeAccessType<'a>
{
    type Item = Value::RelativeType;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub trait TreeIteratorTrait<'a, Value>
where Value: KnowsRelativeAccessType<'a>
{
    fn iter_tree(self) -> TreeIterator<'a, Value>;
}

impl<'a, Value> TreeIteratorTrait<'a, &'a Value> for &'a Value
where &'a Value: KnowsRelativeAccessType<'a>,
      &'a Value: Into<<&'a Value as KnowsRelativeAccessType<'a>>::RelativeType>,
      <&'a Value as KnowsRelativeAccessType<'a>>::RelativeType: HasBranches<'a> + Clone,
      <<&'a Value as KnowsRelativeAccessType<'a>>::RelativeType as KnowsBranches<'a>>::Branches: Into<<&'a Value as KnowsRelativeAccessType<'a>>::RelativeType>
{
    fn iter_tree(self) -> TreeIterator<'a, &'a Value> {
        TreeIterator::new(self)
    }
}

// TODO: Mutable TreeIterator