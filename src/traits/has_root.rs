pub trait KnowsRoot {
    type Root;
}

pub trait HasRoot: KnowsRoot {
    fn root(self) -> Self::Root;
}

impl KnowsRoot for &String {
    type Root = Self;
}

impl HasRoot for &String {
    fn root(self) -> Self::Root {
        self
    }
}