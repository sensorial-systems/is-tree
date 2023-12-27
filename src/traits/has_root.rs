pub trait KnowsRoot<'a> {
    type Root;
}

pub trait HasRoot<'a>: KnowsRoot<'a> {
    fn root(self) -> Self::Root;
}

impl<'a> KnowsRoot<'a> for &String {
    type Root = Self;
}

impl<'a> HasRoot<'a> for &String {
    fn root(self) -> Self::Root {
        self
    }
}