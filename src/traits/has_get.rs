use crate::KnowsPathSegment;

pub trait KnowsGetType<'a> {
    type GetType: KnowsPathSegment;
}

pub trait HasGet<'a>: KnowsGetType<'a> {
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>;
}