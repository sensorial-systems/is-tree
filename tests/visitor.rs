use enum_as_inner::EnumAsInner;

use ::is_tree::*;

use ::is_tree::knows_parent::KnowsParent;
use ::is_tree::new_visitor::{Visitor, RootVisitor};

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

pub enum ModuleParent<'a> {
    Library(&'a Library),
    Module(&'a Module)
}

impl<'a> From<&'a Library> for ModuleParent<'a> {
    fn from(library: &'a Library) -> Self {
        Self::Library(library)
    }
}

impl<'a> From<&'a Module> for ModuleParent<'a> {
    fn from(module: &'a Module) -> Self {
        Self::Module(module)
    }
}

impl<'a> KnowsParent<'a> for Library {
    type Parent = ();
}

impl<'a> KnowsParent<'a> for Module {
    type Parent = ModuleParent<'a>;
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

#[derive(EnumAsInner)]
enum Visitors<'a> {
    Root(&'a RootVisitor),
    Library(&'a LibraryVisitor<'a>),
    Module(&'a ModuleVisitor<'a>)
}

impl<'a> From<&'a RootVisitor> for Visitors<'a> {
    fn from(visitor: &'a RootVisitor) -> Self {
        Self::Root(visitor)
    }
}

impl<'a> From<&'a LibraryVisitor<'a>> for Visitors<'a> {
    fn from(visitor: &'a LibraryVisitor<'a>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<&'a ModuleVisitor<'a>> for Visitors<'a> {
    fn from(visitor: &'a ModuleVisitor<'a>) -> Self {
        Self::Module(visitor)
    }
}

impl<'a> From<&'a ModuleVisitorParent<'a>> for Visitors<'a> {
    fn from(visitor: &'a ModuleVisitorParent<'a>) -> Self {
        match visitor {
            ModuleVisitorParent::Library(library) => Self::Library(library),
            ModuleVisitorParent::Module(module) => Self::Module(module)
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
    // let b = a.child(b);
    // let c = b.child(c);
    // let d = c.child(d);

    assert_eq!(a.path.to_string(), "a");
    // assert_eq!(b.path.to_string(), "a::b");
    // assert_eq!(c.path.to_string(), "a::b::c");
    // assert_eq!(d.path.to_string(), "a::b::c::d");

    assert_eq!(*a.parent().path_segment(), ());
    // assert_eq!(*b.parent().path_segment(), "a");
    // assert_eq!(*c.parent().path_segment(), "b");
    // assert_eq!(*d.parent().path_segment(), "c");
    // // TODO: Make this possible:
    // // assert_eq!(*c.parent().parent().path_segment(), "a");
    // // assert_eq!(*d.parent().parent().parent().path_segment(), "a");

    // assert_eq!(*a.root().path_segment(), "a");
    // assert_eq!(*b.root().path_segment(), "a");
    // assert_eq!(*c.root().path_segment(), "a");
    // assert_eq!(*d.root().path_segment(), "a");

    // assert_eq!(*a.relative::<Visitors, _>(vec![String::self_() ]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*a.relative::<Visitors, _>(vec![String::root()  ]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*b.relative::<Visitors, _>(vec![String::self_() ]).unwrap().as_module() .unwrap().path_segment(), "b");
    // assert_eq!(*b.relative::<Visitors, _>(vec![String::super_()]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*b.relative::<Visitors, _>(vec![String::root()  ]).unwrap().as_library().unwrap().path_segment(), "a");
    // // TODO: Make it work:
    // // assert_eq!(*c.relative::<Visitors, _>(vec![String::super_(), String::super_()]).unwrap().as_library().unwrap().path_segment(), "a");
}