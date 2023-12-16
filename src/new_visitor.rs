pub mod root_visitor;
pub use root_visitor::*;

use crate::{knows_parent::KnowsParent, has_get::{KnowsGetType, HasGet}, Path};
use crate::traits::*;

#[derive(Clone, Copy, Default)]
pub struct Visitor<Parent, Value>
where Value: HasPathSegment
{
    pub parent: Parent,
    pub value: Value
}


impl<'a, Parent, Value> IsVisitor<'a, Value> for Visitor<Parent, Value>
where Value: HasPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
        where Child: HasPathSegment<PathSegment = <Value as HasPathSegment>::PathSegment>,
              Child: KnowsParentVisitor<'a>,
              Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}

impl<'a, Parent, Value> HasPathSegment for Visitor<Parent, Value>
where Value: HasPathSegment
{
    type PathSegment = Value::PathSegment;
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }
}

impl<'a, Parent, Value> HasRelativeAccessType<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + HasRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}


impl<'a, Value> HasRoot<'a> for Visitor<Value::ParentVisitor, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
      Value::ParentVisitor: HasRoot<'a>
{
    type Root = <Value::ParentVisitor as HasRoot<'a>>::Root;
    fn root(self) -> Self::Root {
        self.parent.root()
    }
}

impl<'a, Parent, Value> KnowsGetType<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + KnowsGetType<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Parent, Value> HasGet<'a> for Visitor<Parent, Value>
where Value: Copy + HasPathSegment + HasGet<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as HasPathSegment>::PathSegment> {
        self.value.get(key).map(|value| self.visit(value))
    }
}

//
// Visitor knows parent
//

impl<'a, Parent, Value> KnowsParent<'a> for Visitor<Parent, Value>
where Value: HasPathSegment,
{
    type Parent = Parent;
}

//
// Visitor has parent
//

impl<'a, Parent, Value> KnowsParentVisitor<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
{
    type ParentVisitor = Value::ParentVisitor;
}

impl<'a, Parent, Value> HasParent<'a> for Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: Copy
{
    fn parent(self) -> Parent {
        self.parent
    }
}

//
// Visitor constructors
//

impl<Parent, Value> Visitor<Parent, Value>
where Value: HasPathSegment
{
    pub fn new_with_parent(parent: Parent, value: Value) -> Self {
        Self { parent, value }
    }
}

impl<Parent, Value> HasPath<Value::PathSegment> for Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: HasPath<Value::PathSegment>
{
    fn path(&self) -> Path<Value::PathSegment>
    {
        let mut path = self.parent.path();
        path.segments.push(self.value.path_segment().clone());
        path
    }

}

impl<'a, Parent, Value> HasRelativeAccess<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + HasRelativeAccessType<'a> + KnowsParentVisitor<'a, ParentVisitor = Parent>,
      Parent: Copy + Into<Self::RelativeType>,
      Self: Into<Self::RelativeType> + HasPathSegment,
      Parent: HasRoot<'a>,
      <Parent as HasRoot<'a>>::Root: Into<Self::RelativeType>,
 
      Value::RelativeType:
      HasRelativeAccess<'a>
    + HasRelativeAccessType<'a, RelativeType = Self::RelativeType>
    + HasPathSegment<PathSegment = Self::PathSegment>
    + HasParent<'a>
    + HasRoot<'a, Root = Self::Root>,
    <Value::RelativeType as KnowsParent<'a>>::Parent: Into<Self::RelativeType>
{}
