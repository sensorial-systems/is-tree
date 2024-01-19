use crate::{KnowsVisitor, KnowsParent, KnowsValue, HasVisitorConstructor};

pub trait IsVisitor<'a>: KnowsParent + KnowsValue {
    fn visit<Child: KnowsVisitor<'a>>(&self, value: Child) -> Child::Visitor
    where Child::Visitor: HasVisitorConstructor<'a, Value = Child>,
          Self: Into<<Child::Visitor as KnowsParent>::Parent> + Clone
    {
        Child::Visitor::new_with_parent(self.clone().into(), value)
    }
}

impl<T> IsVisitor<'_> for T
where T: KnowsParent + KnowsValue {}