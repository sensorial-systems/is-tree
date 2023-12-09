pub trait HasRoot {
    type Root;
    fn root(&self) -> &Self::Root;
}
