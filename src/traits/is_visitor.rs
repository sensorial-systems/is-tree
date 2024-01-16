use crate::{KnowsVisitor, KnowsParent, KnowsValue, HasVisitorConstructor};

pub trait IsVisitor: KnowsParent + KnowsValue {
    fn visit<Child: KnowsVisitor>(&self, value: Child) -> Child::Visitor
    where Child::Visitor: HasVisitorConstructor<Value = Child>,
          Self: Into<<Child::Visitor as KnowsParent>::Parent> + Clone
    {
        Child::Visitor::new_with_parent(self.clone().into(), value)
    }
}

