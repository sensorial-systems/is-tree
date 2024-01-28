use crate::{KnowsParent, KnowsVisitorFor};

/// Reference type iterator.
pub struct TypeIterator<Visitor>
{
    stack: Vec<Visitor>,
}

impl<Visitor> From<Vec<Visitor>> for TypeIterator<Visitor> {
    fn from(stack: Vec<Visitor>) -> Self {
        Self { stack }
    }
}

pub trait TypeIter<'a, Visitor: KnowsParent<'a>> {
    fn type_iterator(self, parent: Option<Visitor::Parent>) -> TypeIterator<Visitor>;
}

impl<Visitor> Iterator for TypeIterator<Visitor>
{
    type Item = Visitor;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub trait IterType<'a> {
    fn iter_type<Value>(self) -> TypeIterator<<&'a Value as KnowsVisitorFor<'a, Self>>::Visitor>
    where
        &'a Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, <&'a Value as KnowsVisitorFor<'a, Self>>::Visitor> + Sized,
    {
        self.iter_type_with_parent::<Value>(None)
    }

    fn iter_type_with_parent<Value>(self, parent: Option<<<&'a Value as KnowsVisitorFor<'a, Self>>::Visitor as KnowsParent<'a>>::Parent>) -> TypeIterator<<&'a Value as KnowsVisitorFor<'a, Self>>::Visitor>
    where
        &'a Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, <&'a Value as KnowsVisitorFor<'a, Self>>::Visitor> + Sized,
    {
        self.type_iterator(parent)
    }
}

impl<'a, T> IterType<'a> for T {}
