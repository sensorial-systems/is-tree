pub enum PathSegment<T> {
    Root,
    Self_,
    Super,
    Other(T)
}
