pub trait KnowsRoot<'a> {
    type Root;
}

pub trait HasRoot<'a>: KnowsRoot<'a> {
    fn root(&self) -> Self::Root;
}
