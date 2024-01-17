pub trait KnowsValue {
    type Value;
}

pub trait HasValue: KnowsValue {
    fn value(&self) -> Self::Value;
}
