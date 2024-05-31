use crate::new_traits::*;

pub struct TreeIterator<Visitor> {
    stack: Vec<Visitor>,
}

impl<'a, Visitor> TreeIterator<Visitor>
{
    pub fn mutable<Value>(root: Value) -> Self
    where Value: Into<Visitor>,
          Visitor: Clone + 'a,
          &'a mut Visitor: HasBranches<Visitor>,
    {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.mutable_build(root.into());
        iterator
    }

    fn mutable_build(&mut self, mut visitor: Visitor)
    where Visitor: Clone + 'a,
          &'a mut Visitor: HasBranches<Visitor>,
    {
        #[inline]
        fn longer_ref<'longer, T>(t: &mut T) -> &'longer mut T { unsafe { &mut *(t as *mut T) } }
        self.stack.push(visitor.clone());
        for child in longer_ref(&mut visitor).branches::<Visitor>() {
            let visitor = child.into();
            self.mutable_build(visitor);
        }
    }

    pub fn constant<Value>(root: Value) -> Self
    where Value: Into<Visitor>,
          Visitor: Clone + 'a,
          &'a Visitor: HasBranches<Visitor>,
    {
        let stack = Vec::new();
        let mut iterator = Self { stack };
        iterator.constant_build(root.into());
        iterator
    }

    fn constant_build(&mut self, visitor: Visitor)
    where Visitor: Clone + 'a,
          &'a Visitor: HasBranches<Visitor>,
    {
        #[inline]
        fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
        self.stack.push(visitor.clone());
        for child in longer_ref(&visitor).branches::<Visitor>() {
            let visitor = child.into();
            self.constant_build(visitor);
        }
    }
}

impl<Visitor> Iterator for TreeIterator<Visitor> {
    type Item = Visitor;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
