use crate::{HasGet, HasPathSegment, KnowsBranches, KnowsOwned};

pub trait AddBranch<'a>: KnowsBranches<'a> {
    fn add_branch(&'a mut self, branch: impl Into<<Self::Branches as KnowsOwned>::Owned>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned;
}

pub trait HasGetOrCreate<'a>: AddBranch<'a>
where Self: 'a,
      <Self as KnowsBranches<'a>>::Branches: KnowsOwned,
      &'a mut Self: HasGet<'a, Branches = &'a mut <Self::Branches as KnowsOwned>::Owned>,
      <&'a mut Self as KnowsBranches<'a>>::Branches: HasPathSegment,
      <Self::Branches as KnowsOwned>::Owned: From<String>
{
    fn branch(&'a mut self, segment: impl Into<String>) -> &'a mut <Self::Branches as KnowsOwned>::Owned
    where Self::Branches: KnowsOwned {
        let segment = segment.into();
        let self_ = unsafe { &mut *(self as *mut Self) }; // This is safe. The borrow checker doesn't know that though.
        if let Some(branch) = self.get(segment.clone()) {
            branch
        } else {
            self_.add_branch(<Self::Branches as KnowsOwned>::Owned::from(segment))
        }
    }
}


