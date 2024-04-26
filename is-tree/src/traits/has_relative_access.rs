pub trait KnowsRelativeAccess<'a> {
    type RelativeAccess;
}

pub trait HasRelativeAccess<'a>: KnowsRelativeAccess<'a> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeAccess>
    where K: Into<String>;
}

impl<'a, T: KnowsRelativeAccess<'a>> KnowsRelativeAccess<'a> for &'a T {
    type RelativeAccess = T::RelativeAccess;
}