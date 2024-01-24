use crate::{IsVisitor, KnowsParent};

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

pub trait KnowsVisitorFor<'a, Base> {
    type Visitor: IsVisitor<'a>;
}

pub trait IterType<'a> {
    fn iter_type<Value>(&'a self) -> TypeIterator<Value::Visitor>
    where
        Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, Value::Visitor> + Sized,
    {
        self.iter_type_with_parent::<Value>(None)
    }

    fn iter_type_with_parent<Value>(&'a self, parent: Option<<Value::Visitor as KnowsParent<'a>>::Parent>) -> TypeIterator<Value::Visitor>
    where
        Value: KnowsVisitorFor<'a, Self>,
        Self: TypeIter<'a, Value::Visitor> + Sized,
    {
        self.type_iterator(parent)
    }
}

impl<'a, T> IterType<'a> for T {}











pub struct TypeIterMut<'a, Value>
{
    stack: Vec<&'a mut Value>,
}

impl<'a, Value> From<Vec<&'a mut Value>> for TypeIterMut<'a, Value> {
    fn from(stack: Vec<&'a mut Value>) -> Self {
        Self { stack }
    }
}

pub trait IntoIterTypeMut<Item> {
    fn type_iterator(&mut self) -> TypeIterMut<'_, Item>;
    
}

impl<'a, Value> Iterator for TypeIterMut<'a, Value>
{
    type Item = &'a mut Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}


pub trait IterTypeMut {
    fn iter_type_mut<T>(&mut self) -> TypeIterMut<'_, T>
    where Self: IntoIterTypeMut<T>
    {
        self.type_iterator()
    }
}

impl<T> IterTypeMut for T {}
