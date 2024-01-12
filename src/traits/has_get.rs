use crate::KnowsPathSegment;

pub trait KnowsGetType<'a> {
    type GetType;
}

pub trait HasGet<'a>: KnowsGetType<'a>
where Self::GetType: KnowsPathSegment
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>;
}