pub mod root_visitor;
pub use root_visitor::*;

use crate::{*, knows_parent::KnowsParent, has_get::{KnowsGetType, HasGet}};

#[derive(Clone, Copy, Default)]
pub struct Visitor<Parent, Value>
where Value: HasPathSegment
{
    pub parent: Parent,
    pub value: Value
}


impl<'a, Parent, Value> IsVisitor<'a, Value> for &'a Visitor<Parent, Value>
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

impl<'a, Parent, Value> HasPathSegment for &'a Visitor<Parent, Value>
where Value: HasPathSegment {
    type PathSegment = Value::PathSegment;
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }
}

impl<'a, Parent, Value> HasRelativeAccessType<'a> for &'a Visitor<Parent, Value>
where Value: HasPathSegment + HasRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}

impl<'a, Parent, Value> HasRelativeAccessType<'a> for Visitor<Parent, Value>
where Value: HasPathSegment + HasRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}


impl<'a, Value> HasRoot<'a> for &'a Visitor<Value::ParentVisitor, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
      &'a Value::ParentVisitor: HasRoot<'a>
{
    type Root = <&'a Value::ParentVisitor as HasRoot<'a>>::Root;
    fn root(self) -> Self::Root {
        self.parent.root()
    }
}

impl<'a, Parent, Value> KnowsGetType<'a> for &'a Visitor<Parent, Value>
where Value: HasPathSegment + KnowsGetType<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Parent, Value> HasGet<'a> for &'a Visitor<Parent, Value>
where Value: Copy + HasPathSegment + HasGet<'a>,
      Value::GetType: HasPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as HasPathSegment>::PathSegment> {
        self.value.get(key).map(|value| self.visit(value))
    }
}

pub trait HasRelativeAccessType<'a> {
    type RelativeType;
}

//
// Visitor knows parent
//

impl<'a, Parent, Value> KnowsParent<'a> for &'a Visitor<Parent, Value>
where Value: HasPathSegment,
{
    type Parent = Parent;
}

//
// Visitor has parent
//

impl<'a, Parent, Value> KnowsParentVisitor<'a> for &'a Visitor<Parent, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
{
    type ParentVisitor = Value::ParentVisitor;
}

impl<'a, Parent, Value> HasParent<'a> for &'a Visitor<Parent, Value>
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

pub trait HasRelativeAccess<'a>:
      HasRelativeAccessType<'a>
    + HasPathSegment
    + Into<Self::RelativeType>
    + HasParent<'a>
    + HasRoot<'a>
    where
    Self::Parent: Into<Self::RelativeType>,
    Self::Root: Into<Self::RelativeType>,
    
    Self::RelativeType:
      HasRelativeAccess<'a>
    + HasRelativeAccessType<'a, RelativeType = Self::RelativeType>
    + HasPathSegment<PathSegment = Self::PathSegment>
    + HasParent<'a>
    + HasRoot<'a, Root = Self::Root>,
    <Self::RelativeType as KnowsParent<'a>>::Parent: Into<Self::RelativeType>
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as HasPathSegment>::PathSegment>,
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            match segment.kind() {
                PathSegment::Root => Some(self.root().into()),
                PathSegment::Self_ => self.relative(path),
                PathSegment::Super => self.parent().into().relative(path),
                _ => todo!("Not implemented yet")
                // Identifier::Super => self
                //     .parent
                //     .as_ref()
                //     .and_then(|parent| parent.relative(path)),
                // Identifier::Other(segment) => self
                //     .value
                //     .get(segment.clone())
                //     .and_then(|branch|
                //         self.child(branch)
                //             .relative(path)
                //     )
            }
        } else {
            Some(self.into())
        }
    }
}

impl<'a, Parent, Value> HasRelativeAccess<'a> for &'a Visitor<Parent, Value>
where Value: HasPathSegment + HasRelativeAccessType<'a> + KnowsParentVisitor<'a, ParentVisitor = Parent>,
      Parent: Copy + Into<Self::RelativeType>,
      Self: Into<Self::RelativeType> + HasPathSegment,
      &'a Parent: HasRoot<'a>,
      <&'a Parent as HasRoot<'a>>::Root: Into<Self::RelativeType>,
 
      Value::RelativeType:
      HasRelativeAccess<'a>
    + HasRelativeAccessType<'a, RelativeType = Self::RelativeType>
    + HasPathSegment<PathSegment = Self::PathSegment>
    + HasParent<'a>
    + HasRoot<'a, Root = Self::Root>,
    <Value::RelativeType as KnowsParent<'a>>::Parent: Into<Self::RelativeType>
{}
