use crate::{HasPathSegment, Path, traits::{has_root::HasRoot, has_parent::HasParent, is_tree::IsTree}};

#[derive(Clone)]
pub struct Visitor<'a, Parent, Value>
where Value: HasPathSegment + Clone, Parent: Clone
{
    pub parent: &'a Parent,
    pub value: &'a Value,
    pub path: Path<'a, Value::PathSegment>
}

impl<'a, Value> Visitor<'a, (), Value>
where Value: HasPathSegment + Clone
{
    pub fn new(value: &'a Value) -> Self {
        let path = Path::default().join(value.path_segment().clone());
        let parent = &();
        Self { value, parent, path }
    }
}


impl<'a, Value> HasRoot for Visitor<'a, (), Value>
where Value: HasPathSegment + Clone
{
    type Root = Self;
    fn root(&self) -> &Self {
        self
    }
}


impl<'a, Parent, Value> HasRoot for Visitor<'a, Parent, Value>
where Value: HasPathSegment + Clone,
      Parent: HasRoot + Clone
{
    type Root = Parent::Root;
    fn root(&self) -> &Self::Root {
        self.parent.root()
    }
}

impl<'a, Parent, Value> HasParent for Visitor<'a, Parent, Value>
where Value: HasPathSegment + Clone, Parent: Clone
{
    type Parent = Parent;
    fn parent(&self) -> &Self::Parent {
        self.parent
    }
}

impl<'a, Parent, Value> Visitor<'a, Parent, Value>
where Value: HasPathSegment + Clone, Parent: Clone
{
    pub fn new_with_parent(value: &'a Value, parent: &'a Parent) -> Self {
        let path = Path::default().join(value.path_segment().clone());
        Self { value, parent, path }
    }

    pub fn new_with_parent_and_path(value: &'a Value, parent: &'a Parent, path: Path<'a, Value::PathSegment>) -> Self {
        let path = path.join(value.path_segment().clone());
        Self { value, parent, path }
    }

    pub fn child<Child>(&'a self, value: &'a Child) -> Visitor<'a, Self, Child>
    where Child: HasPathSegment<PathSegment = Value::PathSegment> + Clone
    {
        Visitor::new_with_parent_and_path(value, self, self.path.clone())
    }

    pub fn relative<K, RParent, RValue>(&self, path: impl IntoIterator<Item = K>) -> Option<Visitor<'a, RParent, RValue>>
    where K: Into<Value::PathSegment>,
        Parent: HasRoot,
        Parent::Root: Clone + Into<Visitor<'a, RParent, RValue>>,
        Value: IsTree,
        RParent: Clone,
        RValue: HasPathSegment + Clone,
        Visitor<'a, Parent, Value>: Into<Visitor<'a, RParent, RValue>>
    {
        let mut path = path.into_iter();
        if let Some(_segment) = path.next() {
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
    use crate::*;

    use super::Visitor;

    #[derive(Clone)]
    pub struct Library {
        name: String,
        root_module: Module
    }

    impl HasPathSegment for Library {
        type PathSegment = String;
        fn path_segment(&self) -> &Self::PathSegment {
            &self.name
        }
    }

    #[derive(Clone)]
    struct Module {
        name: String,
        children: Vec<Module>
    }

    impl HasPathSegment for Module {
        type PathSegment = String;
        fn path_segment(&self) -> &Self::PathSegment {
            &self.name
        }
    }

    #[test]
    fn new_visitor() {
        let library = Library {
            name: String::from("a"),
            root_module: Module {
                name: String::from("b"),
                children: vec![
                    Module {
                        name: String::from("c"),
                        children: vec![
                            Module {
                                name: String::from("d"),
                                children: vec![]
                            }
                        ]
                    }
                ]
            }
        };
        let a = &library;
        let b = &a.root_module;
        let c = &b.children[0];
        let d = &c.children[0];
        let a = Visitor::new(a);
        let b = a.child(b);
        let c = b.child(c);
        let d = c.child(d);
        
        assert_eq!(a.path.to_string(), "a");
        assert_eq!(b.path.to_string(), "a::b");
        assert_eq!(c.path.to_string(), "a::b::c");
        assert_eq!(d.path.to_string(), "a::b::c::d");

        assert_eq!(*a.parent(), ());
        assert_eq!(*b.parent().value.path_segment(), String::from("a"));
        assert_eq!(*c.parent().value.path_segment(), String::from("b"));
        assert_eq!(*d.parent().value.path_segment(), String::from("c"));

        assert_eq!(*a.root().value.path_segment(), String::from("a"));
        assert_eq!(*b.root().value.path_segment(), String::from("a"));
        assert_eq!(*c.root().value.path_segment(), String::from("a"));
        assert_eq!(*d.root().value.path_segment(), String::from("a"));
    }
}