use crate::HasPathSegment;

pub trait KnowsGetType<'a> {
    type GetType: HasPathSegment;
}

pub trait HasGet<'a>: KnowsGetType<'a> {
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as HasPathSegment>::PathSegment>;
}