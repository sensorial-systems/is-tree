use crate::{*, knows_parent::KnowsParent};

#[derive(Clone, Copy, Default)]
pub struct Visitor<Parent, Value>
where Value: HasPathSegment
{
    pub parent: Parent,
    pub value: Value
}

impl<Parent, Value> IsVisitor for Visitor<Parent, Value>
where Value: HasPathSegment
{}

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

pub type RootVisitor<Value> = Visitor<(), Value>;


impl<'a, Value> HasRoot<'a> for &'a Visitor<Value::ParentVisitor, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
      &'a Value::ParentVisitor: HasRoot<'a>
{
    type Root = <&'a Value::ParentVisitor as HasRoot<'a>>::Root;
    fn root(self) -> Self::Root {
        self.parent.root()
    }
}

impl<'a, Value> HasRoot<'a> for RootVisitor<Value>
where Value: HasPathSegment
{
    type Root = Self;
    fn root(self) -> Self {
        self
    }
}


impl<'a, Value> HasRelativeAccess<'a> for RootVisitor<Value>
where Value: HasPathSegment
{
    fn relative<RelativeType, K>(self, _path: impl IntoIterator<Item = K>) -> Option<RelativeType>
        where K: Into<<Self as HasPathSegment>::PathSegment>,
              Self: HasRoot<'a>,
              Self: Into<RelativeType>,
              Self: HasParent<'a>,
              <Self as KnowsParent<'a>>::Parent: Into<RelativeType>,
              <Self as HasRoot<'a>>::Root: Into<RelativeType>
    {
        Some(self.into())
    }
}

impl<'a, Value> RootVisitor<Value>
where Value: HasPathSegment, Value::PathSegment: Default
{
    pub fn new(value: Value) -> Self {
        let parent = Default::default();
        Self { parent, value }
    }
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

impl<'a, Parent, Value> HasParent<'a> for &'a Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: Clone
{
    fn parent(self) -> Parent {
        self.parent.clone()
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

    pub fn visit<'a, Child>(&'a self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: HasPathSegment<PathSegment = Value::PathSegment>,
          Child: KnowsParentVisitor<'a>,
          &'a Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent(self.into(), value)
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

pub trait HasRelativeAccess<'a>: HasPathSegment /* + HasParent<'a> */ {
    fn relative<RelativeType, K>(self, path: impl IntoIterator<Item = K>) -> Option<RelativeType>
    where K: Into<<Self as HasPathSegment>::PathSegment>,
          // TODO: Move these things to the trait constraints.
          Self: HasRoot<'a>,
          Self: Into<RelativeType>,
          Self: HasParent<'a>,
          <Self as KnowsParent<'a>>::Parent: Into<RelativeType>,
          <Self as HasRoot<'a>>::Root: Into<RelativeType>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            match segment.kind() {
                PathSegment::Root => Some(self.root().into()),
                PathSegment::Self_ => self.relative(path),
                PathSegment::Super => {
                    Some(self.parent().into())
                },
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

// impl<'a, Parent, Value> HasRelativeAccess<'a> for &'a Visitor<Parent, Value>
// where Value: HasPathSegment + KnowsParentVisitor<'a>,
//       Self: HasPathSegment {}

// impl<'a, Parent, Value> HasRelativeAccess<'a> for Visitor<Parent, Value>
// where Value: HasPathSegment + KnowsParentVisitor<'a>,
//       Self: HasPathSegment {}