pub trait KnowsRoot<'a> {
    type Root;
}

pub trait HasRoot<'a>: KnowsRoot<'a> {
    fn root(self) -> Self::Root;
}

impl<'a, T: KnowsRoot<'a>> KnowsRoot<'a> for &'a T {
    type Root = T::Root;
}

impl<'a, T: KnowsRoot<'a>> KnowsRoot<'a> for &'a mut T {
    type Root = T::Root;
}