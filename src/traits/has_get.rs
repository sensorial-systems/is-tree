use crate::KnowsPathSegment;

pub trait KnowsGetType {
    type GetType;
}

pub trait HasGet: KnowsGetType
where Self::GetType: KnowsPathSegment
{
    fn get<K>(&self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>;
}
