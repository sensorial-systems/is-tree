pub trait KnowsValue {
    type Value;
}

pub trait HasValue: KnowsValue {
    fn value(self) -> Self::Value;
}

// TODO: Remove duplicity
impl<T> KnowsValue for &T
where T: KnowsValue
{
    type Value = T::Value;
}

// TODO: Remove duplicity
impl<T> HasValue for &T
where T: HasValue + Clone
{
    fn value(self) -> Self::Value {
        self.clone().value()
    }
}