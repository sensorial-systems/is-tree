use crate::{KnowsVisitor, KnowsParent, KnowsValue, HasVisitorConstructor};

pub trait IsVisitor<'a>: KnowsParent<'a> + KnowsValue<'a> {
    fn visit<Child: KnowsVisitor<'a>>(&self, value: Child) -> Child::Visitor
    where Child::Visitor: HasVisitorConstructor<'a, Value = Child>,
          Self: Into<<Child::Visitor as KnowsParent<'a>>::Parent> + Clone
    {
        Child::Visitor::new_with_parent(self.clone().into(), value)
    }
}

impl<'a, T> IsVisitor<'a> for T
where T: KnowsParent<'a> + KnowsValue<'a> {}