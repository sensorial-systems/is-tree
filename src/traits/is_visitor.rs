use crate::{KnowsVisitor, KnowsParent, KnowsValue};

pub trait IsVisitor<'a>: Sized {
    fn visit<Child: KnowsVisitor<'a>>(&'a self, value: Child) -> Child::Visitor
    where Child::Visitor: VisitorConstructor<'a, Owned = Child::Visitor, Value = Child>,
          &'a Self: Into<<Child::Visitor as KnowsParent<'a>>::Parent>
    {
        Child::Visitor::new_with_parent(self.into(), value)
    }
}

pub trait VisitorConstructor<'a>: KnowsParent<'a> + KnowsValue<'a> {
    type Owned: KnowsParent<'a> + KnowsValue<'a>;
    fn new_with_parent(parent: <Self::Owned as KnowsParent<'a>>::Parent, value: <Self::Owned as KnowsValue<'a>>::Value) -> Self::Owned;
}