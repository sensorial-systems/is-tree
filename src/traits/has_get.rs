use crate::*;

pub trait KnowsGetType {
    type GetType;
}

pub trait HasGet: KnowsGetType
where Self::GetType: KnowsPathSegment
{
    fn get<PathSegment>(&self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment>;
}

impl<T> KnowsGetType for T
where Self: KnowsValue,
      <Self as KnowsValue>::Value: KnowsGetType,
      <<Self as KnowsValue>::Value as KnowsGetType>::GetType: KnowsVisitor
{
    type GetType = <<<Self as KnowsValue>::Value as KnowsGetType>::GetType as KnowsVisitor>::Visitor;
}

impl<T> HasGet for T
where Self: HasValue + HasParent + Clone,
      <Self as KnowsValue>::Value: Clone + HasGet,
      <<Self as KnowsValue>::Value as KnowsGetType>::GetType: KnowsPathSegment + KnowsVisitor,
      <<<Self as KnowsValue>::Value as KnowsGetType>::GetType as KnowsVisitor>::Visitor: KnowsPathSegment<PathSegment = <<<Self as KnowsValue>::Value as KnowsGetType>::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<Value = <<Self as KnowsValue>::Value as KnowsGetType>::GetType>,
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
