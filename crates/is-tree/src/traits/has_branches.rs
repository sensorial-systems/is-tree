//! This module contains the `HasBranches` trait and its associated API traits.

/// This is the trait one should implement to provide a way to iterate over the branches of a type.
pub trait HasBranches<T> {
    /// This is the method that should be implemented to provide a way to iterate over the branches of a type.
    /// It's discouraged to use this method directly. Instead, use the `branches` method from the `HasBranchesAPIV2` trait.
    fn branches_impl(self) -> impl Iterator<Item = T>;
}    

pub trait HasBranchesAPI {
    /// This is used internally. Should use `branches` instead.
    fn branches_impl2<T>(self) -> impl Iterator<Item = T>
    where Self: HasBranches<T> + Sized
    {
        self.branches_impl()
    }
}

impl<T> HasBranchesAPI for T {}

/// This is the trait one should implement to provide a way to iterate over the branches of a type.
pub trait HasBranchesAPIV2<'a> {
    /// Iterates over the branches of a type.
    fn branches<T>(&'a self) -> impl Iterator<Item = T>
    where &'a Self: HasBranches<T>,
          T: 'a
    {
        self.branches_impl()
    }

    /// Iterates over the branches of a type mutably.
    fn branches_mut<T>(&'a mut self) -> impl Iterator<Item = T>
    where &'a mut Self: HasBranches<T>,
          T: 'a
    {
        self.branches_impl()
    }
}

impl<'a, T> HasBranchesAPIV2<'a> for T {}

/// This is the trait one should implement to provide a way to add a branch to a type.
pub trait AddBranch<T> {
    /// Add a value to the branches of a type.
    fn add_branch(&mut self, value: T) -> &mut T;
}
