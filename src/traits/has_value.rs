pub trait KnowsValue<'a> {
    type Value;
}

pub trait HasValue<'a>: KnowsValue<'a> {
    fn value(&'a self) -> Self::Value;
}

impl<'a, T: KnowsValue<'a>> KnowsValue<'a> for &'a T
{
    type Value = T::Value;
}