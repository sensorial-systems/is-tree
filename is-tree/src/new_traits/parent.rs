pub trait HasParent: Sized {
    fn parent(&self) -> Option<Self>;
}
