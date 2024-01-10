pub mod root_visitor;
use std::rc::Rc;

pub use root_visitor::*;

use crate::{knows_parent::KnowsParent, has_get::{KnowsGetType, HasGet}, Path, PathSegment};
use crate::traits::*;

#[derive(Clone, Default)]
// FIXME: Make this private.
pub struct Internal<Parent, Value> {
    pub parent: Parent,
    pub value: Value
}

#[derive(Clone, Default)]
pub struct Visitor<Parent, Value> {
    // FIXME: Make this private.
    pub internal: Rc<Internal<Parent, Value>>
}

impl<'a, Parent, Value> IsVisitor<'a, Value> for &'a Visitor<Parent, Value>
where Value: KnowsPathSegment
{
    fn visit<Child>(self, value: Child) -> Visitor<Child::ParentVisitor, Child>
        where Child: KnowsPathSegment<PathSegment = <Value as KnowsPathSegment>::PathSegment>,
              Child: KnowsParentVisitor<'a>,
              Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
    }
}

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

impl<'a, Parent, Value> KnowsRelativeAccessType<'a> for Visitor<Parent, Value>
where Value: KnowsRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}

impl<'a, Parent, Value> KnowsRelativeAccessType<'a> for &'a Visitor<Parent, Value>
where Value: KnowsRelativeAccessType<'a>
{
    type RelativeType = Value::RelativeType;
}

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

impl<'a, Parent, Value> KnowsGetType<'a> for Visitor<Parent, Value>
where Value: KnowsGetType<'a>,
      Value::GetType: KnowsParentVisitor<'a>
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Parent, Value> HasGet<'a> for Visitor<Parent, Value>
where Value: Clone + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.internal.value.clone().get(key).map(|value| Visitor::new_with_parent(self.into(), value))
    }
}

impl<'a, Parent, Value> KnowsGetType<'a> for &'a Visitor<Parent, Value>
where Value: KnowsGetType<'a>,
      Value::GetType: KnowsParentVisitor<'a>
{
    type GetType = Visitor<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor, Value::GetType>;
}

impl<'a, Parent, Value> HasGet<'a> for &'a Visitor<Parent, Value>
where Value: Clone + KnowsPathSegment + HasGet<'a>,
      Value::GetType: KnowsPathSegment<PathSegment = Value::PathSegment> + KnowsParentVisitor<'a>,
      Self: Into<<Value::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,
{
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment> {
        self.internal.value.clone().get(key).map(|value| self.visit(value))
    }
}

//
// Visitor knows parent
//

impl<'a, Parent, Value> KnowsParent<'a> for &'a Visitor<Parent, Value> {
    type Parent = Parent;
}

//
// Visitor has parent
//

impl<'a, Parent, Value> KnowsParentVisitor<'a> for Visitor<Parent, Value>
where Value: KnowsParentVisitor<'a>,
{
    type ParentVisitor = Value::ParentVisitor;
}

impl<'a, Parent, Value> HasParent<'a> for &'a Visitor<Parent, Value>
where Parent: Clone
{
    fn parent(self) -> Parent {
        self.internal.parent.clone()
    }
}

//
// Visitor constructors
//

impl<Parent, Value> Visitor<Parent, Value> {
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

impl<'a, Parent, Value> HasRelativeAccess<'a> for &'a Visitor<Parent, Value>
where
    Self: Into<Self::RelativeType> + KnowsPathSegment,
    Parent: Into<Self::RelativeType> + Clone + 'a,
    Value: KnowsPathSegment + KnowsRelativeAccessType<'a> + KnowsParentVisitor<'a, ParentVisitor = Parent> + 'a,

    Self: HasRoot<'a>,
    <Self as KnowsRoot<'a>>::Root: Into<Self::RelativeType>,
    &'a Parent: HasRoot<'a, Root = <Self as KnowsRoot<'a>>::Root>,
    &'a Value::RelativeType: HasRoot<'a, Root = <Self as KnowsRoot<'a>>::Root>,

    Self: HasGet<'a>,
    <Self as KnowsGetType<'a>>::GetType:
        KnowsParentVisitor<'a>
        + Into<Self::RelativeType>
        + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>,
    Self: Into<<<Self as KnowsGetType<'a>>::GetType as KnowsParentVisitor<'a>>::ParentVisitor>,

    <Self as KnowsParent<'a>>::Parent: Into<Self::RelativeType>,
    &'a Value::RelativeType:
      HasRelativeAccess<'a>
    + KnowsRelativeAccessType<'a, RelativeType = Self::RelativeType>
    + KnowsPathSegment<PathSegment = <Self as KnowsPathSegment>::PathSegment>
    + HasParent<'a>,
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>
    {
            let mut path = path.into_iter();
            if let Some(segment) = path.next() {
                let segment = segment.into();
                let visitor = match segment.kind() {
                    PathSegment::Self_ => self.into(),
                    PathSegment::Root => self.root().into(),
                    PathSegment::Super => self.parent().into(),
                    PathSegment::Other(_) => self.get(segment)?.into()
                };
                // FIXME: This is a hack.
                let visitor = unsafe { std::mem::transmute::<_, &'a Value::RelativeType>(&visitor) };
                visitor.relative(path)
            } else {
                Some(self.into())
            }    
    }
}
