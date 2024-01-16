pub trait KnowsRoot {
    type Root;
}

pub trait HasRoot: KnowsRoot {
    fn root(self) -> Self::Root;
}

// TODO: Remove duplicity.
impl<T> KnowsRoot for &T
where T: KnowsRoot
{
    type Root = T::Root;
}
