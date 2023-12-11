use crate::{*, knows_parent::KnowsParent};

#[derive(Clone, Default)]
pub struct Visitor<Parent, Value>
where Value: HasPathSegment
{
    pub parent: Parent,
    pub value: Value,
    pub path: Path<Value::PathSegment>
}

impl<'a, Parent, Value> HasPathSegment for Visitor<Parent, Value>
where Value: HasPathSegment {
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

// TODO: Implement it as RootVisitor<Value: HasPathSegment> = Visitor<(), Value::PathSegment>
pub type RootVisitor = Visitor<(), String>;

impl HasRoot for RootVisitor {
    type Root = Self;
    fn root(self) -> Self {
        self
    }
}

impl<'a> HasRelativeAccess<'a> for RootVisitor
{
    fn relative<RelativeType, K>(self, _path: impl IntoIterator<Item = K>) -> Option<RelativeType>
        where K: Into<<Self as HasPathSegment>::PathSegment>,
              Self: HasRoot,
              Self: Into<RelativeType>,
              Self: HasParent<'a>,
              <Self as KnowsParent<'a>>::Parent: Into<RelativeType>,
              <Self as HasRoot>::Root: Into<RelativeType>
    {
        Some(self.into())
    }
}

impl<'a, Value> Visitor<RootVisitor, Value>
where Value: HasPathSegment
{
    pub fn new(value: Value) -> Self {
        let path = Path::default().join(value.path_segment().clone());
        let parent = Default::default();
        Self { parent, value, path }
    }
}

//
// Visitor has root
// 

impl<'a, Value> HasRoot for &'a Visitor<RootVisitor, Value>
where Value: HasPathSegment
{
    type Root = Self;
    fn root(self) -> Self {
        self
    }
}


impl<'a, Parent, Value> HasRoot for &'a Visitor<Parent, Value>
where Value: HasPathSegment,
      Parent: HasRoot + Copy
{
    type Root = Parent::Root;
    fn root(self) -> Self::Root {
        self.parent.root()
    }
}

//
// Visitor knows parent
//

impl<'a, Value> KnowsParent<'a> for &'a Visitor<Value::ParentVisitor, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
{
    type Parent = Value::ParentVisitor;
}

//
// Visitor has parent
//

impl<'a, Value> HasParent<'a> for &'a Visitor<Value::ParentVisitor, Value>
where Value: KnowsParentVisitor<'a> + HasPathSegment,
      Value::ParentVisitor: Clone
{
    fn parent(self) -> Value::ParentVisitor {
        self.parent.clone()
    }
}

//
// Visitor constructors
//

impl<Parent, Value> Visitor<Parent, Value>
where Value: HasPathSegment
{
    pub fn new_with_parent(value: Value, parent: Parent) -> Self {
        let path = Path::default().join(value.path_segment().clone());
        Self { value, parent, path }
    }

    pub fn new_with_parent_and_path(parent: Parent, value: Value, path: Path<Value::PathSegment>) -> Self {
        let path = path.join(value.path_segment().clone());
        Self { value, parent, path }
    }

    pub fn child<'a, Child>(&'a self, value: Child) -> Visitor<Child::ParentVisitor, Child>
    where Child: HasPathSegment<PathSegment = Value::PathSegment>,
          Child: KnowsParentVisitor<'a>,
          &'a Self: Into<Child::ParentVisitor>
    {
        Visitor::new_with_parent_and_path(self.into(), value, self.path.clone())
    }
}

pub trait HasRelativeAccess<'a>: HasPathSegment {
    fn relative<RelativeType, K>(self, path: impl IntoIterator<Item = K>) -> Option<RelativeType>
    where K: Into<<Self as HasPathSegment>::PathSegment>,
          Self: HasRoot,
          Self: Into<RelativeType>,
          Self: HasParent<'a>,
          <Self as KnowsParent<'a>>::Parent: Into<RelativeType>,
          <Self as HasRoot>::Root: Into<RelativeType>
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

impl<'a, Parent, Value> HasRelativeAccess<'a> for &'a Visitor<Parent, Value>
where Value: HasPathSegment + KnowsParentVisitor<'a>,
      Self: HasPathSegment {}

// impl<'a, Parent, Value> HasRelativeAccess<'a> for Visitor<Parent, Value>
// where Value: HasPathSegment + KnowsParentVisitor<'a>,
//       Self: HasPathSegment {}