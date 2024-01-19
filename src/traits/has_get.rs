use crate::*;

pub trait KnowsGetType<'a> {
    type GetType;
}

pub trait HasGet<'a>: KnowsGetType<'a>
where Self::GetType: KnowsPathSegment
{
    fn get<PathSegment>(&self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment>;
}

impl<'a, T> KnowsGetType<'a> for T
where Self: KnowsValue,
      <Self as KnowsValue>::Value: KnowsGetType<'a>,
      <<Self as KnowsValue>::Value as KnowsGetType<'a>>::GetType: KnowsVisitor<'a>
{
    type GetType = <<<Self as KnowsValue>::Value as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, T> HasGet<'a> for T
where Self: HasValue + HasParent + Clone,
      <Self as KnowsValue>::Value: Clone + HasGet<'a>,
      <<Self as KnowsValue>::Value as KnowsGetType<'a>>::GetType: KnowsPathSegment + KnowsVisitor<'a>,
      <<<Self as KnowsValue>::Value as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <<<Self as KnowsValue>::Value as KnowsGetType<'a>>::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<'a, Value = <<Self as KnowsValue>::Value as KnowsGetType<'a>>::GetType>,
      Self: Into<<Self::GetType as KnowsParent>::Parent>,
{
    fn get<PathSegment>(&self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self
            .value()
            .get(segment)
            .map(|value| self.visit(value))
    }
}
