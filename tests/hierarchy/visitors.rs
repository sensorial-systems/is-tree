use enum_as_inner::EnumAsInner;
use ::is_tree::*;

use super::{LibraryVisitor, ModuleVisitor, ModuleParentVisitor};

#[derive(EnumAsInner)]
pub enum Visitors<'a> {
    Library(LibraryVisitor<'a>),
    Module(ModuleVisitor<'a>)
}

impl<'a> KnowsRoot for &'a Visitors<'a> {
    type Root = LibraryVisitor<'a>;
}

impl<'a> HasRoot for &'a Visitors<'a> {
    fn root(self) -> Self::Root {
        match self {
            Visitors::Library(library) => library.clone(),
            Visitors::Module(module) => module.clone().parent().root()
        }
    }
}

impl<'a> KnowsRelativeAccessType for &'a Visitors<'a> {
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

impl<'a> KnowsGetType for &'a Visitors<'a> {
    type GetType = Visitors<'a>;
}

impl<'a> HasGet for &'a Visitors<'a> {
    fn get<K>(self, key: K) -> Option<Self::GetType>
        where K: Into<<Self::GetType as KnowsPathSegment>::PathSegment>
    {
        match self {
            Visitors::Library(library) => library.get(key).map(|value| value.into()),
            Visitors::Module(module) => module.get(key).map(|value| value.into())
        }
    }
}

impl<'a> HasRelativeAccess for &'a Visitors<'a> {
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

impl<'a> KnowsParent for &'a Visitors<'a> {
    type Parent = Visitors<'a>;
}

impl<'a> HasParent for &'a Visitors<'a> {
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

impl<'a> From<ModuleVisitor<'a>> for Visitors<'a> {
    fn from(visitor: ModuleVisitor<'a>) -> Self {
        Self::Module(visitor)
    }
}
