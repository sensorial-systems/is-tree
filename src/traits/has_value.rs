pub trait KnowsValue<'a> {
    type Value;
}

pub trait HasValue<'a>: KnowsValue<'a> {
    fn value(self) -> Self::Value;
}