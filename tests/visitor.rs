use enum_as_inner::EnumAsInner;

use ::is_tree::*;

use ::is_tree::has_get::{KnowsGetType, HasGet};
use ::is_tree::knows_parent::KnowsParent;
use ::is_tree::new_visitor::{Visitor, RootVisitor, HasRelativeAccess, HasRelativeAccessType};

pub struct Library {
    name: String,
    root_module: Module
}

impl HasRootVisitor for &Library {}

impl<'a> KnowsGetType<'a> for &'a Library {
    type GetType = &'a Module;
}

impl<'a> HasGet<'a> for &'a Library {
    fn get<K>(self, key: K) -> Option<Self::GetType>
    where K: Into<<Self::GetType as HasPathSegment>::PathSegment>
    {
        if &key.into() == self.root_module.path_segment() {
            Some(&self.root_module)
        } else {
            None
        }
    }
}

impl<'a> KnowsGetType<'a> for &'a Module {
    type GetType = &'a Module;
}

impl<'a> HasGet<'a> for &'a Module {
    fn get<K>(self, key: K) -> Option<Self::GetType>
        where K: Into<<Self::GetType as HasPathSegment>::PathSegment>
    {
        let key = key.into();
        self.children.iter().find(|child| &key == child.path_segment())
    }
}

impl HasPathSegment for Library {
    type PathSegment = String;
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl<'a> HasRelativeAccessType<'a> for &'a Module {
    type RelativeType = Visitors<'a>;
}

impl<'a> HasRelativeAccessType<'a> for Library {
    type RelativeType = Visitors<'a>;
}

impl<'a> HasRelativeAccessType<'a> for &'a Library {
    type RelativeType = Visitors<'a>;
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
    Library(LibraryVisitor<'a>),
    Module(&'a ModuleVisitor<'a>)
}

impl<'a> HasPath<String> for ModuleVisitorParent<'a> {
    fn path(&self) -> Path<String> {
        match self {
            ModuleVisitorParent::Library(library) => library.path(),
            ModuleVisitorParent::Module(module) => module.path()
        }
    }
}

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
        Self::Library(*visitor)
    }
}

impl<'a> From<LibraryVisitor<'a>> for ModuleVisitorParent<'a> {
    fn from(visitor: LibraryVisitor<'a>) -> Self {
        Self::Library(visitor)
    }

}

impl<'a> From<&'a ModuleVisitor<'a>> for ModuleVisitorParent<'a> {
    fn from(visitor: &'a ModuleVisitor<'a>) -> Self {
        Self::Module(visitor)
    }
}

impl<'a> HasRoot<'a> for ModuleVisitorParent<'a> {
    type Root = LibraryVisitor<'a>;
    fn root(self) -> Self::Root {
        match self {
            ModuleVisitorParent::Library(library) => library,
            ModuleVisitorParent::Module(module) => module.parent().root()
        }
    }
}

impl<'a> HasRoot<'a> for &'a ModuleVisitorParent<'a> {
    type Root = LibraryVisitor<'a>;
    fn root(self) -> Self::Root {
        match self {
            ModuleVisitorParent::Library(library) => *library,
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
    Library(LibraryVisitor<'a>),
    Module(&'a ModuleVisitor<'a>)
}

impl<'a> HasRoot<'a> for Visitors<'a> {
    type Root = LibraryVisitor<'a>;
    fn root(self) -> Self::Root {
        match self {
            Visitors::Library(library) => library,
            Visitors::Module(module) => module.parent().root()
        }
    }
}

impl<'a> HasRelativeAccessType<'a> for Visitors<'a> {
    type RelativeType = Visitors<'a>;
}

impl<'a> HasRelativeAccess<'a> for Visitors<'a> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
        where K: Into<<Self as HasPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.relative(path),
            Visitors::Module(module) => module.relative(path)
        }
    }
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

impl<'a> From<LibraryVisitor<'a>> for Visitors<'a> {
    fn from(visitor: LibraryVisitor<'a>) -> Self {
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
            ModuleVisitorParent::Library(library) => Self::Library(*library),
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

    assert_eq!(a.path().to_string(), "a");
    assert_eq!(b.path().to_string(), "a::b");
    assert_eq!(c.path().to_string(), "a::b::c");
    assert_eq!(d.path().to_string(), "a::b::c::d");

    assert_eq!(*a.parent().path_segment(), "a"); // Root's parent is itself. Will it create any kind of problem?
    assert_eq!(*b.parent().path_segment(), "a");
    assert_eq!(*c.parent().path_segment(), "b");
    assert_eq!(*d.parent().path_segment(), "c");
    assert_eq!(*c.parent().parent().path_segment(), "a");
    assert_eq!(*d.parent().parent().parent().path_segment(), "a");

    assert_eq!(*a.root().path_segment(), "a");
    assert_eq!(*b.root().path_segment(), "a");
    assert_eq!(*c.root().path_segment(), "a");
    assert_eq!(*d.root().path_segment(), "a");

    assert_eq!(a.get("b").unwrap().get("c").unwrap().path_segment(), "c");

    assert_eq!(*a.relative(vec!["self"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*a.relative(vec!["root"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*b.relative(vec!["self"]).unwrap().as_module() .unwrap().path_segment(), "b");
    assert_eq!(*b.relative(vec!["super"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*b.relative(vec!["root"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*c.relative(vec!["super", "super"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(a.relative(vec!["b", "c"]).unwrap().as_module().unwrap().path_segment(), "c");
}