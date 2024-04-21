use crate::{KnowsVisitor, KnowsParent, HasVisitorConstructor};

pub trait IsVisitor<'a>: KnowsParent<'a> {
    fn visit<Child: KnowsVisitor<'a>>(&self, value: Child) -> Child::Visitor
    where Child::Visitor: HasVisitorConstructor<'a, Value = Child>,
          Self: Into<<Child::Visitor as KnowsParent<'a>>::Parent> + Clone
    {
        Child::Visitor::new(self.clone().into(), value)
    }
}

impl<'a, T> IsVisitor<'a> for T
where T: KnowsParent<'a> {}