pub trait HasBranches<T> {
    fn branches_impl(self) -> impl Iterator<Item = T>;
}    


pub trait HasBranchesAPI {
    fn branches<T>(self) -> impl Iterator<Item = T>
    where Self: HasBranches<T> + Sized
    {
        self.branches_impl()
    }
}

impl<T> HasBranchesAPI for T {}

pub trait AddBranch<T> {
    fn add_branch(&mut self, value: T) -> &mut T;
}
