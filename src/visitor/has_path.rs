use crate::{KnowsPathSegment, Visitor, HasPathSegment, Path, HasPath};

impl<'a, Parent, Value> KnowsPathSegment for Visitor<Parent, Value>
where Value: KnowsPathSegment
{
    type PathSegment = Value::PathSegment;
}

impl<'a, Parent, Value> KnowsPathSegment for &'a Visitor<Parent, Value>
where Value: KnowsPathSegment
{
    type PathSegment = Value::PathSegment;
}

impl<'a, Parent, Value> HasPathSegment for Visitor<Parent, Value>
where Value: HasPathSegment
{
    fn path_segment(&self) -> &Self::PathSegment {
        self.internal.value.path_segment()
    }
}

impl<'a, Parent, Value> HasPath<Value::PathSegment> for Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: HasPath<Value::PathSegment>
{
    fn path(&self) -> Path<Value::PathSegment>
    {
        let mut path = self.internal.parent.path();
        path.segments.push(self.internal.value.path_segment().clone());
        path
    }

}
