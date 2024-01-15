use crate::{KnowsVisitor, KnowsParent, KnowsValue};

pub trait IsVisitor: KnowsParent + KnowsValue {
    fn visit<Child: KnowsVisitor>(&self, value: Child) -> Child::Visitor
    where Child::Visitor: VisitorConstructor<Value = Child>,
          Self: Into<<Child::Visitor as KnowsParent>::Parent> + Clone
    {
        Child::Visitor::new_with_parent(self.clone().into(), value)
    }
}

pub trait VisitorConstructor: KnowsParent + KnowsValue
where Self::Value: KnowsVisitor,
      <Self::Value as KnowsVisitor>::Visitor: KnowsParent,
{
    fn new_with_parent(parent: <<Self::Value as KnowsVisitor>::Visitor as KnowsParent>::Parent, value: Self::Value) -> <Self::Value as KnowsVisitor>::Visitor;
}