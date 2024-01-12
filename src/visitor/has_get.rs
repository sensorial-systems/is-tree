use crate::{has_get::{KnowsGetType, HasGet}, Visitor, KnowsPathSegment, KnowsVisitor, HasValue, KnowsParent, VisitorConstructor, KnowsValue, IsVisitor};

impl<'a, Parent, Value> KnowsGetType<'a> for Visitor<Parent, Value>
where Value: KnowsGetType<'a>,
      Value::GetType: KnowsVisitor<'a>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

// TODO: Check if this is still necessary.
// impl<'a, Parent, Value> HasGet<'a> for Visitor<Parent, Value>
// where Value: Clone + HasGet<'a>,
//       Value::GetType: KnowsPathSegment + KnowsVisitor<'a>,
//       <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::GetType as KnowsPathSegment>::PathSegment>,
//       Self::GetType: VisitorConstructor<'a, Value = Value::GetType> + KnowsParent<'a> + KnowsValue<'a, Value = Value::GetType>,
//       &'a Self: Into<<Self::GetType as KnowsParent<'a>>::Parent> + 'a,
// {
//     fn get<K>(self, key: K) -> Option<Self::GetType>
//     where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
//         self.internal.value.clone().get(key).map(|value|
//             self.visit(value)
//         )
//     }
// }

impl<'a, Parent, Value> KnowsGetType<'a> for &'a Visitor<Parent, Value>
where Value: KnowsGetType<'a> + KnowsPathSegment,
      Value::GetType: KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Parent, Value> HasGet<'a> for &'a Visitor<Parent, Value>
where Value: Clone + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>,
      Self::GetType: VisitorConstructor<'a, Value = Value::GetType> + KnowsParent<'a> + KnowsValue<'a, Value = Value::GetType>,
      Self: Into<<Self::GetType as KnowsParent<'a>>::Parent> + 'a,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.value().get(key).map(|value|
            self.visit(value)
        )
    }
}
