use crate::*;

impl<'a, Parent, Value> KnowsGetType<'a> for Visitor<Parent, Value>
where Value: ToOwned,
      Value::Owned: KnowsGetType<'a>,
      <Value::Owned as KnowsGetType<'a>>::GetType: KnowsVisitor<'a>
{
    type GetType = <<<Value as ToOwned>::Owned as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Parent, Value> HasGet<'a> for Visitor<Parent, Value>
where Parent: Clone,
      Value: Clone + ToOwned + HasGet<'a, GetType = <Value::Owned as KnowsGetType<'a>>::GetType>,
      Value::Owned: KnowsGetType<'a>,
      <Value::Owned as KnowsGetType<'a>>::GetType: KnowsPathSegment + KnowsVisitor<'a>,
      <<Value::Owned as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <<<Value as ToOwned>::Owned as KnowsGetType<'a>>::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: HasVisitorConstructor<'a, Value = <<Value as ToOwned>::Owned as KnowsGetType<'a>>::GetType>,
      Self: Into<<Self::GetType as KnowsParent<'a>>::Parent>,
{
    fn get<PathSegment>(&'a self, segment: PathSegment) -> Option<Self::GetType>
    where PathSegment: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self
            .internal.value // TODO: How to use .value() here instead?
            .get(segment)
            .map(|value| self.visit(value))
    }
}
