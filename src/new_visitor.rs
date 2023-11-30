use crate::{IsIdentifier, HasIdentifier, Path, Identifier, IsTree};

#[derive(Clone)]
pub struct Visitor<'a, Parent, Value>
where Value: HasIdentifier + Clone, Parent: Clone
{
    pub parent: &'a Parent,
    pub value: &'a Value,
    pub path: Path<'a, Value::Identifier>
}

impl<'a, Value> Visitor<'a, (), Value>
where Value: HasIdentifier + Clone
{
    pub fn new(value: &'a Value) -> Self {
        let path = Path::default().join(value.identifier().clone());
        let parent = &();
        Self { value, parent, path }
    }
}

pub trait HasParent {
    type Parent;
    fn parent(&self) -> &Self::Parent;
}

pub trait HasRoot {
    type Root;
    fn root(&self) -> &Self::Root;
}

impl<'a, Value> HasRoot for Visitor<'a, (), Value>
where Value: HasIdentifier + Clone
{
    type Root = Self;
    fn root(&self) -> &Self {
        self
    }
}


impl<'a, Parent, Value> HasRoot for Visitor<'a, Parent, Value>
where Value: HasIdentifier + Clone,
      Parent: HasRoot + Clone
{
    type Root = Parent::Root;
    fn root(&self) -> &Self::Root {
        self.parent.root()
    }
}

impl<'a, Parent, Value> HasParent for Visitor<'a, Parent, Value>
where Value: HasIdentifier + Clone, Parent: Clone
{
    type Parent = Parent;
    fn parent(&self) -> &Self::Parent {
        self.parent
    }
}

impl<'a, Parent, Value> Visitor<'a, Parent, Value>
where Value: HasIdentifier + Clone, Parent: Clone
{
    pub fn new_with_parent(value: &'a Value, parent: &'a Parent) -> Self {
        let path = Path::default().join(value.identifier().clone());
        Self { value, parent, path }
    }

    pub fn new_with_parent_and_path(value: &'a Value, parent: &'a Parent, path: Path<'a, Value::Identifier>) -> Self {
        let path = path.join(value.identifier().clone());
        Self { value, parent, path }
    }

    pub fn child<Child>(&'a self, value: &'a Child) -> Visitor<'a, Self, Child>
    where Child: HasIdentifier<Identifier = Value::Identifier> + Clone
    {
        Visitor::new_with_parent_and_path(value, self, self.path.clone())
    }

    pub fn relative<K, RParent, RValue>(&self, path: impl IntoIterator<Item = K>) -> Option<Visitor<'a, RParent, RValue>>
    where K: Into<Value::Identifier>,
        Parent: HasRoot,
        Parent::Root: Clone + Into<Visitor<'a, RParent, RValue>>,
        Value: IsTree,
        RParent: Clone,
        RValue: HasIdentifier + Clone,
        Visitor<'a, Parent, Value>: Into<Visitor<'a, RParent, RValue>>
    {
        let mut path = path.into_iter();
        if let Some(segment) = path.next() {
            Some((*self.root()).clone().into())
            // let segment = segment.into();
            // match segment.kind() {
            //     Identifier::Root => Some(self.root()),
            //     Identifier::Self_ => self.relative(path),
            //     Identifier::Super => self
            //         .parent
            //         .as_ref()
            //         .and_then(|parent| parent.relative(path)),
            //     Identifier::Other(segment) => self
            //         .value
            //         .get(segment.clone())
            //         .and_then(|branch|
            //             self.child(branch)
            //                 .relative(path)
            //         )
            // }
        } else {
            Some((*self).clone().into())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{new_visitor::{HasParent, HasRoot}, Path, IsIdentifier};

    use super::Visitor;


    struct Module {
        name: String,
        children: Vec<Module>
    }



    #[test]
    fn new_visitor() {
        let module = Module {
            name: String::from("a"),
            children: vec![
                Module {
                    name: String::from("b"),
                    children: vec![
                        Module {
                            name: String::from("c"),
                            children: vec![]
                        }
                    ]
                }
            ]
        };
        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");
        // TODO: Implement iterator for visit.
        let a = Visitor::new(&a);
        let b = a.child(&b);
        let c = b.child(&c);
        assert_eq!(a.path.to_string(), "a");
        assert_eq!(b.path.to_string(), "a::b");
        assert_eq!(c.path.to_string(), "a::b::c");
        assert_eq!(*a.parent(), ());
        assert_eq!(*b.parent().value, String::from("a"));
        assert_eq!(*c.parent().value, String::from("b"));
        assert_eq!(*a.root().value, String::from("a"));
        assert_eq!(*b.root().value, String::from("a"));
        assert_eq!(*c.root().value, String::from("a"));
    }
}