use crate::{*, knows_parent::KnowsParent};

pub struct Visitor<'a, Parent, Value>
where Value: HasPathSegment
{
    pub parent: Parent,
    pub value: &'a Value,
    pub path: Path<'a, Value::PathSegment>
}

impl<'a, Parent, Value> Clone for Visitor<'a, Parent, Value>
where Value: HasPathSegment,
      Parent: Clone
{
    fn clone(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            value: self.value,
            path: self.path.clone()
        }
    }

}

impl<'a, Parent, Value> HasPathSegment for Visitor<'a, Parent, Value>
where Value: HasPathSegment {
    type PathSegment = Value::PathSegment;
    fn path_segment(&self) -> &Self::PathSegment {
        self.value.path_segment()
    }

}

impl Default for Visitor<'static, &'static (), ()> {
    fn default() -> Self {
        Visitor {
            parent: &(),
            value: &(),
            path: Path::default()
        }
    }
}

pub type RootVisitor = Visitor<'static, &'static (), ()>;

lazy_static::lazy_static! {
    pub static ref ROOT_VISITOR: RootVisitor = Default::default();
}

impl<'a, Value> Visitor<'a, &'a Visitor<'a, &'a (), ()>, Value>
where Value: HasPathSegment
{
    pub fn new(value: &'a Value) -> Self {
        let path = Path::default().join(value.path_segment().clone());
        let parent = &ROOT_VISITOR;
        Self { parent, value, path }
    }
}

impl<'a, Value> HasRoot for Visitor<'a, &'a Visitor<'a, &'a (), ()>, Value>
where Value: HasPathSegment
{
    type Root = Self;
    fn root(&self) -> &Self {
        self
    }
}


impl<'a, Parent, Value> HasRoot for Visitor<'a, Parent, Value>
where Value: HasPathSegment,
      Parent: HasRoot
{
    type Root = Parent::Root;
    fn root(&self) -> &Self::Root {
        self.parent.root()
    }
}

impl<'a, Parent, Value> KnowsParent<'a> for Visitor<'a, Parent, Value>
where Value: HasPathSegment
{
    type Parent = Parent;
}

impl<'a, Parent, Value> HasParent<'a> for Visitor<'a, Parent, Value>
where Value: HasPathSegment
{
    fn parent(&self) -> &Self::Parent {
        &self.parent
    }
}

pub trait HasVisitorParent<'a> {
    type VisitorParent;
}

impl<'a, Parent, Value> Visitor<'a, Parent, Value>
where Value: HasPathSegment
{
    pub fn new_with_parent(value: &'a Value, parent: Parent) -> Self {
        let path = Path::default().join(value.path_segment().clone());
        Self { value, parent, path }
    }

    pub fn new_with_parent_and_path(value: &'a Value, parent: Parent, path: Path<'a, Value::PathSegment>) -> Self {
        let path = path.join(value.path_segment().clone());
        Self { value, parent, path }
    }

    pub fn child<Child>(&'a self, value: &'a Child) -> Visitor<'a, Child::VisitorParent, Child>
    where Child: HasPathSegment<PathSegment = Value::PathSegment> + HasVisitorParent<'a>,
          &'a Self: Into<Child::VisitorParent>
    {
        Visitor::new_with_parent_and_path(value, self.into(), self.path.clone())
    }
}

impl<'a, Parent, Value> Visitor<'a, Parent, Value>
where Value: HasPathSegment
{
    pub fn relative<RelativeType, K>(&'a self, path: impl IntoIterator<Item = K>) -> Option<RelativeType>
    where K: Into<Value::PathSegment>,
          &'a Self: Into<RelativeType>,
          Value: HasVisitorParent<'a>,
          &'a Value::VisitorParent: Into<RelativeType>,
          Self: HasRoot,
          &'a <Self as HasRoot>::Root: Into<RelativeType>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            let segment = segment.into();
            match segment.kind() {
                PathSegment::Root => Some(self.root().into()),
                PathSegment::Self_ => self.relative(path),
                PathSegment::Super => {
                    // TODO: Make it safer.
                    let result: &Value::VisitorParent = unsafe { std::mem::transmute(&self.parent) };
                    Some(result.into())
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
