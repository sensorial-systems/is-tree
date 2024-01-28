use crate::{KnowsPathSegment, Visitor, HasPathSegment, Path, HasPath};

impl<Parent, Value> KnowsPathSegment for Visitor<Parent, Value>
where Value: KnowsPathSegment
{
    type PathSegment = Value::PathSegment;
}

impl<Parent, Value> HasPathSegment for Visitor<Parent, Value>
where Value: HasPathSegment + Clone
{
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }
}

impl<Parent, Value> HasPath for Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: HasPath<PathSegment = Value::PathSegment>
{
    fn path(&self) -> Path<Value::PathSegment>
    {
        let mut path = self.parent.path();
        path.segments.push(self.value.path_segment().clone());
        path
    }
}
