use crate::{KnowsRoot, Visitor, KnowsParentVisitor, HasRoot};

impl<'a, Value> KnowsRoot<'a> for Visitor<Value::ParentVisitor, Value>
where Value: KnowsParentVisitor<'a>,
      Value::ParentVisitor: KnowsRoot<'a> + Clone
{
    type Root = <Value::ParentVisitor as KnowsRoot<'a>>::Root;
}

impl<'a, Value> KnowsRoot<'a> for &'a Visitor<Value::ParentVisitor, Value>
where Value: KnowsParentVisitor<'a>,
      Value::ParentVisitor: KnowsRoot<'a> + Clone
{
    type Root = <Value::ParentVisitor as KnowsRoot<'a>>::Root;
}

impl<'a, Value> HasRoot<'a> for &'a Visitor<Value::ParentVisitor, Value>
where Value: KnowsParentVisitor<'a>,
      Value::ParentVisitor: HasRoot<'a> + Clone
{
    fn root(self) -> Self::Root {
        self.internal.parent.clone().root()
    }
}
