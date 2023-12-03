use crate::*;

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

impl IsPathSegment for () {
    fn root() -> Self {
        ()
    }
    fn self_() -> Self {
        ()
    }
    fn super_() -> Self {
        ()
    }
}

impl HasPathSegment for () {
    type PathSegment = ();
    fn path_segment(&self) -> &Self::PathSegment {
        self
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

lazy_static::lazy_static! {
    pub static ref ROOT_VISITOR: Visitor<'static, &'static (), ()> = Default::default();
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

impl<'a, Parent, Value> HasParent for Visitor<'a, Parent, Value>
where Value: HasPathSegment
{
    type Parent = Parent;
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

impl<'a, Parent, Value> Visitor<'a, &'a Parent, Value>
where Value: HasPathSegment
{

    // pub fn relative<K, RParent, RValue>(&self, path: impl IntoIterator<Item = K>) -> Option<Visitor<'a, RParent, RValue>>
    // where K: Into<Value::PathSegment>,
    //     RValue: HasPathSegment,
    //     Visitor<'a, Parent, Value>: Into<Visitor<'a, RParent, RValue>>
    // {
    //     let mut path = path.into_iter();
    //     if let Some(segment) = path.next() {
    //         let segment = segment.into();
    //         match segment.kind() {
                // PathSegment::Root => Some((*self.root()).clone().into()),
                // PathSegment::Self_ => self.relative(path),
                // TODO: Fix this by implementing relative for Visitor<'a, Visitor<Grandparent, Parent>, Value>
                // PathSegment::Super => self.parent.relative(path),
                // _ => todo!("Hello")
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
        //     }
        // } else {
        //     Some((*self).clone().into())
        // }
    // }
}

#[cfg(test)]
mod test {
    use crate::*;

    use super::{Visitor, HasVisitorParent};

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

    pub struct Module {
        name: String,
        children: Vec<Module>
    }

    impl HasPathSegment for Module {
        type PathSegment = String;
        fn path_segment(&self) -> &Self::PathSegment {
            &self.name
        }
    }

    impl<'a> HasVisitorParent<'a> for Module {
        type VisitorParent = ModuleVisitorParent<'a>;
    }

    type LibraryVisitor<'a> = Visitor<'a, &'a Visitor<'a, &'a (), ()>, Library>;
    type ModuleVisitor<'a> = Visitor<'a, ModuleVisitorParent<'a>, Module>;
    pub enum ModuleVisitorParent<'a> {
        Library(&'a LibraryVisitor<'a>),
        Module(&'a ModuleVisitor<'a>)
    }

    impl<'a> From<&'a LibraryVisitor<'a>> for ModuleVisitorParent<'a> {
        fn from(visitor: &'a LibraryVisitor<'a>) -> Self {
            Self::Library(visitor)
        }
    }

    impl<'a> From<&'a ModuleVisitor<'a>> for ModuleVisitorParent<'a> {
        fn from(visitor: &'a ModuleVisitor<'a>) -> Self {
            Self::Module(visitor)
        }
    }

    impl<'a> HasRoot for ModuleVisitorParent<'a> {
        type Root = LibraryVisitor<'a>;
        fn root(&self) -> &Self::Root {
            match self {
                ModuleVisitorParent::Library(library) => library.root(),
                ModuleVisitorParent::Module(module) => module.root()
            }
        }
    }

    impl<'a> HasPathSegment for ModuleVisitorParent<'a> {
        type PathSegment = String;
        fn path_segment(&self) -> &Self::PathSegment {
            match self {
                ModuleVisitorParent::Library(library) => library.path_segment(),
                ModuleVisitorParent::Module(module) => module.path_segment()
            }
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

        assert_eq!(*a.parent().path_segment(), ());
        assert_eq!(*b.parent().path_segment(), String::from("a"));
        assert_eq!(*c.parent().path_segment(), String::from("b"));
        assert_eq!(*d.parent().path_segment(), String::from("c"));

        assert_eq!(*a.root().path_segment(), String::from("a"));
        assert_eq!(*b.root().path_segment(), String::from("a"));
        assert_eq!(*c.root().path_segment(), String::from("a"));
        assert_eq!(*d.root().path_segment(), String::from("a"));

        assert_eq!(*a.parent().value, ());

        // TODO: Change constraints to make it work.
        // assert_eq!(*a.relative(vec![String::self_()]).unwrap().value.path_segment(), String::from("a"));
        // assert_eq!(*b.relative(vec![String::super_()]).unwrap().value.path_segment(), String::from("a"));

        // TODO: Test it dynamically (everything is statically typed here).
        // TODO: How to create ModuleParent?
    }
}