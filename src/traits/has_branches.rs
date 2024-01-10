use crate::{HasPathSegment, TreeUpdate};

pub trait KnowsBranches<'a> {
    type Branches;
}

pub type BranchesIterator<Branches> = Box<dyn Iterator<Item = Branches>>;

pub trait HasBranches<'a, T>: TreeUpdate<T> + KnowsBranches<'a>
where T: Sized + HasPathSegment
{
    fn branches(&'a self) -> Box<dyn Iterator<Item = &'a Self::Branches> + 'a>;
    fn branches_mut(&'a mut self) -> Box<dyn Iterator<Item = &'a mut Self::Branches> + 'a>;

    // fn branch<K>(&mut self, key: K) -> &mut T
    // where K: Into<T::PathSegment>,
    //       T: From<T::PathSegment>
    // {
    //     // This works and it's safe, but the borrow checker doesn't like it.
    //     // https://rust-lang.github.io/rfcs/2094-nll.html#problem-case-3-conditional-control-flow-across-functions
    //     let myself = unsafe { &mut *(self as *mut Self) };
    //     let key = key.into();
    //     if let Some(value) = myself.get_mut(key.clone()) {
    //         value
    //     } else {
    //         self.add_branch(T::from(key))
    //     }
    // }

    // fn get<K>(&self, key: K) -> Option<&T>
    // where K: Into<T::PathSegment> {
    //     let key = key.into();
    //     self
    //         .branches()
    //         .find(|branch| branch.path_segment() == &key)
    // }
    
    // fn get_mut<K>(&mut self, key: K) -> Option<&mut T>
    // where K: Into<T::PathSegment> {
    //     let key = key.into();
    //     self
    //         .branches_mut()
    //         .find(|branch| branch.path_segment() == &key)
    // }
}
