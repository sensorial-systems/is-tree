pub trait HasBranches<T> {
    fn branches_impl(self) -> impl Iterator<Item = T>;
}    


pub trait HasBranchesAPI {
    fn branches_impl2<T>(self) -> impl Iterator<Item = T>
    where Self: HasBranches<T> + Sized
    {
        self.branches_impl()
    }
}

impl<T> HasBranchesAPI for T {}

pub trait HasBranchesAPIV2<'a> {
    fn branches<T>(&'a self) -> impl Iterator<Item = T>
    where &'a Self: HasBranches<T>,
          T: 'a
    {
        self.branches_impl()
    }

    fn branches_mut<T>(&'a mut self) -> impl Iterator<Item = T>
    where &'a mut Self: HasBranches<T>,
          T: 'a
    {
        self.branches_impl()
    }
}

impl<'a, T> HasBranchesAPIV2<'a> for T {}

pub trait AddBranch<T> {
    fn add_branch(&mut self, value: T) -> &mut T;
}
