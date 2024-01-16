use crate::{KnowsParent, KnowsValue, KnowsVisitor};

pub trait HasVisitorConstructor: KnowsParent + KnowsValue
where Self::Value: KnowsVisitor,
      <Self::Value as KnowsVisitor>::Visitor: KnowsParent,
{
    fn new_with_parent(parent: <<Self::Value as KnowsVisitor>::Visitor as KnowsParent>::Parent, value: Self::Value) -> <Self::Value as KnowsVisitor>::Visitor;
}