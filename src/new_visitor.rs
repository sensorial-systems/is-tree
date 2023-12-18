pub mod root_visitor;
use std::rc::Rc;

pub use root_visitor::*;

use crate::{knows_parent::KnowsParent, has_get::{KnowsGetType, HasGet}, Path, PathSegment};
use crate::traits::*;

#[derive(Clone, Default)]
struct Internal<Parent, Value> {
    parent: Parent,
    value: Value
}

#[derive(Clone, Default)]
pub struct Visitor<Parent, Value> {
    internal: Rc<Internal<Parent, Value>>
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
        self.internal.value.path_segment()
    }
}

impl<'a, Parent, Value> KnowsRelativeAccessType<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + KnowsRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}


impl<'a, Value> HasRoot<'a> for Visitor<Value::ParentVisitor, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
      Value::ParentVisitor: HasRoot<'a> + Clone
{
    type Root = <Value::ParentVisitor as HasRoot<'a>>::Root;
    fn root(self) -> Self::Root {
        self.internal.parent.clone().root()
    }
}

impl<'a, Parent, Value> KnowsGetType<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + KnowsGetType<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Parent, Value> HasGet<'a> for Visitor<Parent, Value>
where Value: Clone + HasPathSegment + HasGet<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as HasPathSegment>::PathSegment> {
        self.internal.value.clone().get(key).map(|value| self.visit(value))
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
      Parent: Clone
{
    fn parent(self) -> Parent {
        self.internal.parent.clone()
    }
}

//
// Visitor constructors
//

impl<Parent, Value> Visitor<Parent, Value>
where Value: HasPathSegment
{
    pub fn new_with_parent(parent: Parent, value: Value) -> Self {
        let internal = Rc::new(Internal { parent, value });
        Self { internal }
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

impl<'a, Parent, Value> HasRelativeAccess<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + KnowsRelativeAccessType<'a> + KnowsParentVisitor<'a, ParentVisitor = Parent>,
      Parent: Clone + Into<Self::RelativeType>,
      Self: Into<Self::RelativeType> + HasPathSegment + HasRoot<'a> + HasGet<'a>,
      <Self as KnowsGetType<'a>>::GetType:
      Into<Self::RelativeType>
      + HasPathSegment<PathSegment = Self::PathSegment>,
      <Self as HasRoot<'a>>::Root: Into<Self::RelativeType>,
      Parent: HasRoot<'a>,
      <Parent as HasRoot<'a>>::Root: Into<Self::RelativeType>,
 
      Value::RelativeType:
      HasRelativeAccess<'a>
    + KnowsRelativeAccessType<'a, RelativeType = Self::RelativeType>
    + HasPathSegment<PathSegment = <Self as HasPathSegment>::PathSegment>
    + HasParent<'a>
    + HasRoot<'a, Root = <Self as HasRoot<'a>>::Root>,
    <Value::RelativeType as KnowsParent<'a>>::Parent: Into<Self::RelativeType>
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as HasPathSegment>::PathSegment>
    {
            let mut path = path.into_iter();
            if let Some(segment) = path.next() {
                let segment = segment.into();
                match segment.kind() {
                    PathSegment::Root => Some(self.root().into()),
                    PathSegment::Self_ => self.relative(path),
                    PathSegment::Super => self.parent().into().relative(path),
                    PathSegment::Other(_segment) => self.get(segment).and_then(|value| value.into().relative(path))
                }
            } else {
                Some(self.into())
            }    
    }
}
