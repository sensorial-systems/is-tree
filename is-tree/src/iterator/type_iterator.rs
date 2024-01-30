use crate::{KnowsParent, KnowsVisitorOf};

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
    fn iter_type<Value>(self) -> TypeIterator<<Self as KnowsVisitorOf<'a, Value>>::Visitor>
    where
        Self: KnowsVisitorOf<'a, Value>,
        Self: TypeIter<'a, <Self as KnowsVisitorOf<'a, Value>>::Visitor> + Sized,
    {
        self.iter_type_with_parent::<Value>(None)
    }

    fn iter_type_with_parent<Value>(self, parent: Option<<<Self as KnowsVisitorOf<'a, Value>>::Visitor as KnowsParent<'a>>::Parent>) -> TypeIterator<<Self as KnowsVisitorOf<'a, Value>>::Visitor>
    where
        Self: KnowsVisitorOf<'a, Value>,
        Self: TypeIter<'a, <Self as KnowsVisitorOf<'a, Value>>::Visitor> + Sized,
    {
        self.type_iterator(parent)
    }
}

impl<'a, T> IterType<'a> for T {}
