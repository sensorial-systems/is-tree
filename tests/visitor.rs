use enum_as_inner::EnumAsInner;

use ::is_tree::*;

use ::is_tree::has_get::{KnowsGetType, HasGet};
use ::is_tree::knows_parent::KnowsParent;
use ::is_tree::new_visitor::{Visitor, RootVisitor};

// use ::is_tree::traits::*;

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
    where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
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
        where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        let key = key.into();
        self.children.iter().find(|child| &key == child.path_segment())
    }
}

impl KnowsPathSegment for Library {
    type PathSegment = String;
}

impl HasPathSegment for Library {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl<'a> KnowsRelativeAccessType<'a> for &'a Module {
    type RelativeType = Visitors<'a>;
}

impl<'a> KnowsRelativeAccessType<'a> for Library {
    type RelativeType = Visitors<'a>;
}

impl<'a> KnowsRelativeAccessType<'a> for &'a Library {
    type RelativeType = Visitors<'a>;
}

impl KnowsPathSegment for &Library {
    type PathSegment = String;
}

impl HasPathSegment for &Library {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

pub struct Module {
    name: String,
    children: Vec<Module>
}

impl KnowsPathSegment for Module {
    type PathSegment = String;
}

impl HasPathSegment for Module {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

impl KnowsPathSegment for &Module {
    type PathSegment = String;
}

impl HasPathSegment for &Module {
    fn path_segment(&self) -> &Self::PathSegment {
        &self.name
    }
}

#[derive(Clone)]
pub enum ModuleParentVisitor<'a> {
    Library(LibraryVisitor<'a>),
    Module(ModuleVisitor<'a>)
}

impl<'a> From<LibraryVisitor<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: LibraryVisitor<'a>) -> Self {
        Self::Library(visitor)
    }
}

// impl<'a> From<&'a ModuleParentVisitor<'a>> for ModuleParentVisitor<'a> {
//     fn from(visitor: &'a ModuleParentVisitor<'a>) -> Self {
//         visitor.clone()
//     }
// }

impl<'a> From<ModuleVisitor<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: ModuleVisitor<'a>) -> Self {
        Self::Module(visitor)
    }
}

impl<'a> From<&'a ModuleVisitor<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: &'a ModuleVisitor<'a>) -> Self {
        Self::Module(visitor.clone())
    }
}

impl<'a> HasPath<String> for ModuleParentVisitor<'a> {
    fn path(&self) -> Path<String> {
        match self {
            ModuleParentVisitor::Library(library) => library.path(),
            ModuleParentVisitor::Module(module) => module.path()
        }
    }
}

impl<'a> KnowsPathSegment for ModuleParentVisitor<'a> {
    type PathSegment = String;
}

impl<'a> HasPathSegment for ModuleParentVisitor<'a> {
    fn path_segment(&self) -> &Self::PathSegment {
        match self {
            ModuleParentVisitor::Library(library) => library.path_segment(),
            ModuleParentVisitor::Module(module) => module.path_segment()
        }
    }
}

impl<'a> KnowsRoot<'a> for ModuleParentVisitor<'a> {
    type Root = LibraryVisitor<'a>;
}

impl<'a> KnowsRoot<'a> for &'a ModuleParentVisitor<'a> {
    type Root = LibraryVisitor<'a>;
}

impl<'a> HasRoot<'a> for ModuleParentVisitor<'a> {
    fn root(self) -> Self::Root {
        match self {
            ModuleParentVisitor::Library(library) => library,
            ModuleParentVisitor::Module(module) => module.parent().root()
        }
    }
}

impl<'a> HasRoot<'a> for &'a ModuleParentVisitor<'a> {
    fn root(self) -> Self::Root {
        match self {
            ModuleParentVisitor::Library(library) => library.clone(),
            ModuleParentVisitor::Module(module) => module.clone().parent().root()
        }
    }
}

impl<'a> KnowsParent<'a> for &'a ModuleParentVisitor<'a> {
    type Parent = ModuleParentVisitor<'a>;
}

impl<'a> HasParent<'a> for &'a ModuleParentVisitor<'a> {
    fn parent(self) -> ModuleParentVisitor<'a> {
        match self {
            ModuleParentVisitor::Library(library) => ModuleParentVisitor::Library(library.clone()),
            ModuleParentVisitor::Module(module) => module.clone().parent().clone()
        }
    }
}

impl<'a> KnowsParentVisitor<'a> for &'a Module {
    type ParentVisitor = ModuleParentVisitor<'a>;
}

impl<'a> KnowsParentVisitor<'a> for &'a Library {
    type ParentVisitor = LibraryVisitor<'a>;
}

type LibraryVisitor<'a> = RootVisitor<&'a Library>;
type ModuleVisitor<'a> = Visitor<ModuleParentVisitor<'a>, &'a Module>;

#[derive(EnumAsInner)]
pub enum Visitors<'a> {
    Library(LibraryVisitor<'a>),
    Module(ModuleVisitor<'a>)
}

impl<'a> KnowsRoot<'a> for &'a Visitors<'a> {
    type Root = LibraryVisitor<'a>;
}

impl<'a> HasRoot<'a> for &'a Visitors<'a> {
    fn root(self) -> Self::Root {
        match self {
            Visitors::Library(library) => library.clone(),
            Visitors::Module(module) => module.clone().parent().root()
        }
    }
}

impl<'a> KnowsRelativeAccessType<'a> for &'a Visitors<'a> {
    type RelativeType = Visitors<'a>;
}

impl<'a> KnowsPathSegment for &'a Visitors<'a> {
    type PathSegment = String;
}

impl<'a> HasPathSegment for &'a Visitors<'a> {
    fn path_segment(&self) -> &Self::PathSegment {
        match self {
            Visitors::Library(library) => library.path_segment(),
            Visitors::Module(module) => module.path_segment()
        }
    }
}

impl<'a> KnowsPathSegment for Visitors<'a> {
    type PathSegment = String;
}

impl<'a> HasPathSegment for Visitors<'a> {
    fn path_segment(&self) -> &Self::PathSegment {
        match self {
            Visitors::Library(visitor) => visitor.path_segment(),
            Visitors::Module(visitor) => visitor.path_segment()
        }
    }
}

impl<'a> KnowsGetType<'a> for &'a Visitors<'a> {
    type GetType = Visitors<'a>;
}

impl<'a> HasGet<'a> for &'a Visitors<'a> {
    fn get<K>(self, key: K) -> Option<Self::GetType>
        where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.get(key).map(|value| value.into()),
            Visitors::Module(module) => module.get(key).map(|value| value.into())
        }
    }
}

impl<'a> HasRelativeAccess<'a> for &'a Visitors<'a> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
        where K: Into<<Self as KnowsPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.relative(path),
            Visitors::Module(module) => module.relative(path)
        }
    }
}

impl<'a> From<ModuleParentVisitor<'a>> for Visitors<'a> {
    fn from(visitor: ModuleParentVisitor<'a>) -> Self {
        match visitor {
            ModuleParentVisitor::Library(library) => Self::Library(library),
            ModuleParentVisitor::Module(module) => Self::Module(module)
        }
    }
}

impl<'a> KnowsParent<'a> for &'a Visitors<'a> {
    type Parent = Visitors<'a>;
}

impl<'a> HasParent<'a> for &'a Visitors<'a> {
    fn parent(self) -> Visitors<'a> {
        match self {
            Visitors::Library(visitor) => visitor.into(),
            Visitors::Module(visitor) => visitor.parent().into()
        }
    }
}

impl<'a> From<&LibraryVisitor<'a>> for Visitors<'a> {
    fn from(visitor: &LibraryVisitor<'a>) -> Self {
        Self::Library(visitor.clone())
    }
}

impl<'a> From<LibraryVisitor<'a>> for Visitors<'a> {
    fn from(visitor: LibraryVisitor<'a>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<&ModuleVisitor<'a>> for Visitors<'a> {
    fn from(visitor: &ModuleVisitor<'a>) -> Self {
        Self::Module(visitor.clone())
    }
}

impl<'a> From<ModuleVisitor<'a>> for Visitors<'a> {
    fn from(visitor: ModuleVisitor<'a>) -> Self {
        Self::Module(visitor)
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

    let visitors = Visitors::from(a.clone());

    assert_eq!(*visitors.parent().path_segment(), "a");
    assert_eq!(*a.parent().path_segment(), "a"); // Root's parent is itself. Will it create any kind of problem?
    assert_eq!(*b.parent().path_segment(), "a");
    assert_eq!(*c.parent().path_segment(), "b");
    assert_eq!(*d.parent().path_segment(), "c");
    assert_eq!(*c.parent().parent().path_segment(), "a");
    assert_eq!(*d.parent().parent().parent().path_segment(), "a");

    assert_eq!(*visitors.root().path_segment(), "a");
    assert_eq!(*a.root().path_segment(), "a");
    assert_eq!(*b.root().path_segment(), "a");
    assert_eq!(*c.root().path_segment(), "a");
    assert_eq!(*d.root().path_segment(), "a");

    assert_eq!(a.get("b").unwrap().get("c").unwrap().path_segment(), "c");
    assert_eq!(visitors.get("b").unwrap().get("c").unwrap().path_segment(), "c");

    assert_eq!(*a.relative(vec!["super"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*a.relative(vec!["self"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*a.relative(vec!["root"]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*a.relative(vec!["b"]).unwrap().as_module().unwrap().path_segment(), "b");
    
    assert_eq!(*b.relative(vec!["self"]).unwrap().as_module() .unwrap().path_segment(), "b");
    assert_eq!(*b.relative(vec!["super"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*b.relative(vec!["root"]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*b.relative(vec!["c"]).unwrap().as_module() .unwrap().path_segment(), "c");
    assert_eq!(*c.relative(vec!["super", "super"]).unwrap().as_library().unwrap().path_segment(), "a");
    // assert_eq!(*a.relative(vec!["b", "c"]).unwrap().as_module().unwrap().path_segment(), "c");
}
