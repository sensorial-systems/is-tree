use std::borrow::BorrowMut;

use crate::TreeVisitor;

use super::has_identifier::HasPathSegment;

pub trait IsTree: HasPathSegment {
    fn is(&self, identifier: impl PartialEq<Self::PathSegment>) -> bool {
        identifier.eq(self.path_segment())
    }

    fn add_branch(&mut self, _child: impl Into<Self>) -> &mut Self where Self: Sized {
        self
    }

    fn remove_branch(&mut self, _identifier: &Self::PathSegment) {}

    fn branches<'a>(&'a self) -> Box<dyn Iterator<Item = &Self> + 'a>;
    fn branches_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut Self> + 'a>;

    fn branch<K>(&mut self, key: K) -> &mut Self
    where K: Into<Self::PathSegment>,
          Self::PathSegment: BorrowMut<Self::PathSegment>,
          Self: From<Self::PathSegment>
    {
        // This works and it's safe, but the borrow checker doesn't like it.
        // https://rust-lang.github.io/rfcs/2094-nll.html#problem-case-3-conditional-control-flow-across-functions
        let myself = unsafe { &mut *(self as *mut Self) };
        let key = key.into();
        if let Some(value) = myself.get_mut(key.clone()) {
            value
        } else {
            self.add_branch(Self::from(key))
        }
    }

    fn get<K>(&self, key: K) -> Option<&Self>
    where K: Into<Self::PathSegment> {
        let key = key.into();
        self
            .branches()
            .find(|branch| branch.path_segment() == &key)
    }
    
    fn get_mut<K>(&mut self, key: K) -> Option<&mut Self>
    where K: Into<Self::PathSegment> {
        let key = key.into();
        self
            .branches_mut()
            .find(|branch| branch.path_segment() == &key)
    }
    
    fn path_get<K>(&self, path: impl IntoIterator<Item = K>) -> Option<&Self>
    where K: Into<Self::PathSegment>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            self
                .get(segment)
                .and_then(|branch|
                    branch.path_get(path)
                )
        } else {
            Some(self)
        }
    }

    fn path_get_mut<K>(&mut self, path: impl IntoIterator<Item = K>) -> Option<&mut Self>
    where K: Into<Self::PathSegment> {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            self
                .get_mut(segment)
                .and_then(|branch|
                    branch.path_get_mut(path)
                )
        } else {
            Some(self)
        }
    }

    fn iter(&self) -> TreeVisitor<'_, Self>
    where Self: Sized
    {
        TreeVisitor::new(self)
    }
}