use crate::{KnowsPathSegment, RootVisitor, HasPathSegment, HasPath, Path};

impl<Value> KnowsPathSegment for RootVisitor<Value>
where Value: KnowsPathSegment
{
    type PathSegment = Value::PathSegment;
}

impl<Value> HasPathSegment for RootVisitor<Value>
where Value: HasPathSegment
{
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }
}

impl<Value> HasPath for RootVisitor<Value>
where Value: HasPathSegment
{
    fn path(&self) -> Path<Value::PathSegment>
    {
        let mut path = Path::default();
        path.segments.push(self.value.path_segment().clone());
        path
    }
}
