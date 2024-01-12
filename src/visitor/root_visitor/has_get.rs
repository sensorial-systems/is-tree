use crate::{has_get::{KnowsGetType, HasGet}, RootVisitor, KnowsPathSegment, KnowsVisitor, IsVisitor, KnowsValue, KnowsParent, VisitorConstructor};

impl<'a, Value> KnowsGetType<'a> for RootVisitor<Value>
where Value: KnowsPathSegment + KnowsGetType<'a>,
      Value::GetType: KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

// impl<'a, Value> HasGet<'a> for RootVisitor<Value>
// where Value: Copy + KnowsPathSegment + HasGet<'a>,
//       Value::GetType: KnowsVisitor<'a> + KnowsPathSegment<PathSegment = Value::PathSegment>,
//       <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = Value::PathSegment>,
//       Self::GetType: VisitorConstructor<'a, Owned = Self::GetType> + KnowsParent<'a> + KnowsValue<'a, Value = Value::GetType>,
//       &'a Self: Into<<Self::GetType as KnowsParent<'a>>::Parent> + 'a,
//       <<Value as KnowsGetType<'a>>::GetType as KnowsVisitor<'a>>::Visitor: KnowsParent<'a>
// {
//     fn get<K>(self, key: K) -> Option<Self::GetType>
//     where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
//         self.value.get(key).map(|value|
//             self.visit(value)
//         )
//     }
// }

impl<'a, Value> KnowsGetType<'a> for &'a RootVisitor<Value>
where Value: KnowsGetType<'a>,
      Value::GetType: KnowsVisitor<'a>
{
    type GetType = <Value::GetType as KnowsVisitor<'a>>::Visitor;
}

impl<'a, Value> HasGet<'a> for &'a RootVisitor<Value>
where Value: Copy + HasGet<'a>,
      Value::GetType: KnowsPathSegment + KnowsVisitor<'a>,
      <Value::GetType as KnowsVisitor<'a>>::Visitor: KnowsPathSegment<PathSegment = <Value::GetType as KnowsPathSegment>::PathSegment>,
      Self::GetType: VisitorConstructor<'a, Value = Value::GetType> + KnowsParent<'a> + KnowsValue<'a, Value = Value::GetType>,
      Self: Into<<Self::GetType as KnowsParent<'a>>::Parent> + 'a,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.value.get(key).map(|value| {
            self.visit(value)
        })
    }
}
