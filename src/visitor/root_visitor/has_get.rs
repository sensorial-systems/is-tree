use crate::*;

impl<'a, Value> KnowsGetType<'a> for RootVisitor<Value>
where Value: ToOwned,
      Value::Owned: KnowsGetType<'a>,
      <Value::Owned as KnowsGetType<'a>>::GetType: KnowsVisitor<'a>
{
    type GetType = <<<Value as ToOwned>::Owned as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasGet<'a> for RootVisitor<Value>
where Value: Clone + ToOwned + HasGet<'a, GetType = <Value::Owned as KnowsGetType<'a>>::GetType>,
      Value::Owned: KnowsGetType<'a>,
      <Value::Owned as KnowsGetType<'a>>::GetType: KnowsPathSegment + KnowsVisitor<'a>,
      <<Value::Owned as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <<<Value as ToOwned>::Owned as KnowsGetType<'a>>::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<'a, Value = <<Value as ToOwned>::Owned as KnowsGetType<'a>>::GetType>,
      Self: Into<<Self::GetType as KnowsParent<'a>>::Parent>,
{
    fn get<PathSegment>(&'a self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self
            .value
            .get(segment)
            .map(|value| self.visit(value))
    }
}
