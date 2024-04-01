pub trait KnowsRelativeAccessType<'a> {
    type RelativeType;
}

pub trait HasRelativeAccess<'a>: KnowsRelativeAccessType<'a> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<String>;
}

impl<'a, T: KnowsRelativeAccessType<'a>> KnowsRelativeAccessType<'a> for &'a T {
    type RelativeType = T::RelativeType;
}