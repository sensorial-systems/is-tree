use crate::{HasPathSegment, Path, HasPath};

#[derive(Clone, Debug, Default)]
pub struct Visitor<Parent, Value> {
    pub parent: Parent,
    pub value: Value
}

impl<Parent, Value> Visitor<Parent, Value> {
    pub fn new(parent: Parent, value: Value) -> Self {
        Self { parent, value }
    }
}

impl<Parent, Value> HasPathSegment for Visitor<Parent, Value>
where Value: HasPathSegment
{
    fn path_segment(&self) -> String {
        self.value.path_segment()
    }
}

impl<Parent, Value> HasPath for Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: HasPath
{
    fn path(&self) -> Path
    {
        let mut path = self.parent.path();
        path.segments.push(self.value.path_segment());
        path
    }
}
