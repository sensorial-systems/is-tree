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
    fn type_iterator(&'a self, parent: Option<Visitor::Parent>) -> TypeIterator<Visitor>;
}

impl<Visitor> Iterator for TypeIterator<Visitor>
{
    type Item = Visitor;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub trait IterType<'a> {
    fn iter_type<Value>(&'a self) -> TypeIterator<<&'a Value as KnowsVisitorFor<'a, Self>>::Visitor>
    where
        &'a Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, <&'a Value as KnowsVisitorFor<'a, Self>>::Visitor> + Sized,
    {
        self.iter_type_with_parent::<Value>(None)
    }

    fn iter_type_with_parent<Value>(&'a self, parent: Option<<<&'a Value as KnowsVisitorFor<'a, Self>>::Visitor as KnowsParent<'a>>::Parent>) -> TypeIterator<<&'a Value as KnowsVisitorFor<'a, Self>>::Visitor>
    where
        &'a Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, <&'a Value as KnowsVisitorFor<'a, Self>>::Visitor> + Sized,
    {
        self.type_iterator(parent)
    }

    fn iter_type_mut<Value>(&'a mut self) -> TypeIterator<<&'a mut Value as KnowsVisitorFor<'a, Self>>::Visitor>
    where
        &'a mut Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, <&'a mut Value as KnowsVisitorFor<'a, Self>>::Visitor> + Sized,
    {
        self.iter_type_mut_with_parent::<Value>(None)
    }

    fn iter_type_mut_with_parent<Value>(&'a mut self, parent: Option<<<&'a mut Value as KnowsVisitorFor<'a, Self>>::Visitor as KnowsParent<'a>>::Parent>) -> TypeIterator<<&'a mut Value as KnowsVisitorFor<'a, Self>>::Visitor>
    where
        &'a mut Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, <&'a mut Value as KnowsVisitorFor<'a, Self>>::Visitor> + Sized,
    {
        todo!()
        // self.type_iterator_mut(parent)
    }
}

impl<'a, T> IterType<'a> for T {}
