pub trait KnowsValue<'a> {
    type Value;
}

pub trait HasValue<'a>: KnowsValue<'a> {
    fn value(self) -> Self::Value;
}

impl<'a, T: KnowsValue<'a>> KnowsValue<'a> for &'a T
{
    type Value = T::Value;
}

impl<'a, T: KnowsValue<'a>> KnowsValue<'a> for &'a mut T
{
    type Value = T::Value;
}