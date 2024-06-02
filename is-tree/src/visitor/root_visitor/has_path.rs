use crate::{RootVisitor, HasPathSegment, HasPath, Path};

impl<Value> HasPathSegment for RootVisitor<Value>
where Value: HasPathSegment
{
    fn path_segment(&self) -> String {
        self.value.path_segment()
    }
}

impl<Value> HasPath for RootVisitor<Value>
where Value: HasPathSegment
{
    fn path(&self) -> Path
    {
        let mut path = Path::default();
        path.segments.push(self.value.path_segment());
        path
    }
}
