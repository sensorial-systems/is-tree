pub trait HasRoot<'a> {
    type Root;
    fn root(self) -> Self::Root;
}

impl<'a> HasRoot<'a> for &String {
    type Root = Self;
    fn root(self) -> Self::Root {
        self
    }
}