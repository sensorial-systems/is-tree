pub trait KnowsRoot {
    type Root;
}

pub trait HasRoot: KnowsRoot {
    fn root(&self) -> Self::Root;
}
