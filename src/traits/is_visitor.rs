use crate::{KnowsVisitor, KnowsParent, KnowsValue};

pub trait IsVisitor<'a>: KnowsParent<'a> + KnowsValue<'a> {
    fn visit<Child: KnowsVisitor<'a>>(&self, value: Child) -> Child::Visitor
    where Child::Visitor: VisitorConstructor<'a, Value = Child>,
          Self: Into<<Child::Visitor as KnowsParent<'a>>::Parent> + Clone
    {
        Child::Visitor::new_with_parent(self.clone().into(), value)
    }
}

pub trait VisitorConstructor<'a>: KnowsParent<'a> + KnowsValue<'a>
where Self::Value: KnowsVisitor<'a>,
      <Self::Value as KnowsVisitor<'a>>::Visitor: KnowsParent<'a>,
{
    fn new_with_parent(parent: <<Self::Value as KnowsVisitor<'a>>::Visitor as KnowsParent<'a>>::Parent, value: Self::Value) -> <Self::Value as KnowsVisitor<'a>>::Visitor;
}