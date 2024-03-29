use crate::{KnowsParent, KnowsValue, KnowsVisitor};

pub trait HasVisitorConstructor<'a>: KnowsParent<'a> + KnowsValue<'a>
where Self::Value: KnowsVisitor<'a>,
      <Self::Value as KnowsVisitor<'a>>::Visitor: KnowsParent<'a>,
{
    fn new_with_parent(parent: <<Self::Value as KnowsVisitor<'a>>::Visitor as KnowsParent<'a>>::Parent, value: Self::Value) -> <Self::Value as KnowsVisitor<'a>>::Visitor;
}