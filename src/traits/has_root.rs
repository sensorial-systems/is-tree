pub trait KnowsRoot {
    type Root;
}

pub trait HasRoot: KnowsRoot {
    fn root(self) -> Self::Root;
}

impl<'a> KnowsRoot for &String {
    type Root = Self;
}

impl<'a> HasRoot for &String {
    fn root(self) -> Self::Root {
        self
    }
}