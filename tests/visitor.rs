use enum_as_inner::EnumAsInner;

use ::is_tree::*;

use ::is_tree::knows_parent::KnowsParent;
use ::is_tree::new_visitor::{Visitor, RootVisitor};

pub struct Library {
    name: String,
    root_module: Module
}

impl HasRootVisitor for &Library {}

impl HasPathSegment for Library {
    type PathSegment = String;
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl HasPathSegment for &Library {
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

impl HasPathSegment for &Module {
    type PathSegment = String;
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl<'a> KnowsParentVisitor<'a> for &Module {
    type ParentVisitor = ModuleVisitorParent<'a>;
}

impl<'a> KnowsParentVisitor<'a> for &'a Library {
    type ParentVisitor = RootVisitor<&'a Library>;
}

type LibraryVisitor<'a> = RootVisitor<&'a Library>;
type ModuleVisitor<'a> = Visitor<ModuleVisitorParent<'a>, &'a Module>;

#[derive(Clone, Copy)]
pub enum ModuleVisitorParent<'a> {
    Library(&'a LibraryVisitor<'a>),
    Module(&'a ModuleVisitor<'a>)
}

// TODO: Enable this:
// impl<'a> HasRelativeAccess<'a> for ModuleVisitorParent<'a>
// {
//     fn relative<RelativeType, K>(self, path: impl IntoIterator<Item = K>) -> Option<RelativeType>
//         where K: Into<<Self as HasPathSegment>::PathSegment>,
//               Self: HasRoot,
//               Self: Into<RelativeType>,
//               Self: HasParent<'a>,
//               <Self as KnowsParent<'a>>::Parent: Into<RelativeType>,
//               <Self as HasRoot>::Root: Into<RelativeType>,
//     {
//         match self {
//             ModuleVisitorParent::Library(library) => library.relative(path),
//             ModuleVisitorParent::Module(module) => module.relative(path)
//         }
//     }
// }

impl<'a> KnowsParent<'a> for ModuleVisitorParent<'a> {
    type Parent = Visitors<'a>;
}

impl<'a> HasParent<'a> for ModuleVisitorParent<'a> {
    fn parent(self) -> Visitors<'a> {
        match self {
            ModuleVisitorParent::Library(library) => library.into(),
            ModuleVisitorParent::Module(module) => module.parent().into()
        }
    }
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

impl<'a> HasRoot<'a> for ModuleVisitorParent<'a> {
    type Root = &'a LibraryVisitor<'a>;
    fn root(self) -> Self::Root {
        match self {
            ModuleVisitorParent::Library(library) => library,
            ModuleVisitorParent::Module(module) => module.parent().root()
        }
    }
}

impl<'a> HasRoot<'a> for &'a ModuleVisitorParent<'a> {
    type Root = &'a LibraryVisitor<'a>;
    fn root(self) -> Self::Root {
        match self {
            ModuleVisitorParent::Library(library) => library,
            ModuleVisitorParent::Module(module) => module.parent().root()
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
pub enum Visitors<'a> {
    Library(&'a LibraryVisitor<'a>),
    Module(&'a ModuleVisitor<'a>)
}

impl<'a> KnowsParent<'a> for Visitors<'a> {
    type Parent = Visitors<'a>;
}

impl<'a> HasParent<'a> for Visitors<'a> {
    fn parent(self) -> Visitors<'a> {
        match self {
            Visitors::Library(visitor) => visitor.into(),
            Visitors::Module(visitor) => visitor.parent().into()
        }
    }
}

impl<'a> HasPathSegment for Visitors<'a> {
    type PathSegment = String;
    fn path_segment(&self) -> &Self::PathSegment {
        match self {
            Visitors::Library(visitor) => visitor.path_segment(),
            Visitors::Module(visitor) => visitor.path_segment()
        }
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

impl<'a> From<ModuleVisitorParent<'a>> for Visitors<'a> {
    fn from(visitor: ModuleVisitorParent<'a>) -> Self {
        match visitor {
            ModuleVisitorParent::Library(library) => Self::Library(library),
            ModuleVisitorParent::Module(module) => Self::Module(module)
        }
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
    let a: LibraryVisitor = a.visit();
    let b: ModuleVisitor = a.visit(b);
    let c: ModuleVisitor = b.visit(c);
    let d: ModuleVisitor = c.visit(d);

    assert_eq!(a.path.to_string(), "a");
    assert_eq!(b.path.to_string(), "a::b");
    assert_eq!(c.path.to_string(), "a::b::c");
    assert_eq!(d.path.to_string(), "a::b::c::d");

    assert_eq!(*a.parent().path_segment(), ""); // TODO: Can this be ()? Do this after the FIXME bellow.
    assert_eq!(*b.parent().path_segment(), "a");
    assert_eq!(*c.parent().path_segment(), "b");
    assert_eq!(*d.parent().path_segment(), "c");
    assert_eq!(*c.parent().parent().path_segment(), "a");
    assert_eq!(*d.parent().parent().parent().path_segment(), "a");

    assert_eq!(*a.root().path_segment(), "a"); // FIXME: This should be "a", not "".
    assert_eq!(*b.root().path_segment(), "a");
    assert_eq!(*c.root().path_segment(), "a");
    assert_eq!(*d.root().path_segment(), "a");

    // assert_eq!(*a.relative::<Visitors, _>(vec![String::self_() ]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*a.relative::<Visitors, _>(vec![String::root()  ]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*b.relative::<Visitors, _>(vec![String::self_() ]).unwrap().as_module() .unwrap().path_segment(), "b");
    // assert_eq!(*b.relative::<Visitors, _>(vec![String::super_()]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*b.relative::<Visitors, _>(vec![String::root()  ]).unwrap().as_library().unwrap().path_segment(), "a");
    // // TODO: Make it work:
    // // assert_eq!(*c.relative::<Visitors, _>(vec![String::super_(), String::super_()]).unwrap().as_library().unwrap().path_segment(), "a");
}