use crate::{KnowsParent, KnowsValue, KnowsVisitor};

pub trait HasVisitorConstructor<'a>: KnowsParent + KnowsValue
where Self::Value: KnowsVisitor<'a>,
      <Self::Value as KnowsVisitor<'a>>::Visitor: KnowsParent,
{
    fn new_with_parent(parent: <<Self::Value as KnowsVisitor<'a>>::Visitor as KnowsParent>::Parent, value: Self::Value) -> <Self::Value as KnowsVisitor<'a>>::Visitor;
}