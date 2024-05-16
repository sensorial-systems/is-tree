use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner, IsTree)]
#[tree(dev)]
pub enum Visitors<Library, Module> {
    Library(LibraryVisitor<Library>),
    Module(Box<Visitor<Visitors<Library, Module>, Module>>)
}

impl<'a, Library, Module> From<LibraryVisitor<Library>> for Visitors<Library, Module> {
    fn from(visitor: LibraryVisitor<Library>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a, Library, Module> From<RootVisitor<&'a mut Library>> for Visitors<&'a Library, Module> {
    fn from(visitor: RootVisitor<&'a mut Library>) -> Self {
        Self::Library(visitor.into())
    }
}

impl<'a, Library, Module> From<ModuleVisitor<Library, Module>> for Visitors<Library, Module> {
    fn from(visitor: ModuleVisitor<Library, Module>) -> Self {
        Self::Module(visitor.into())
    }
}

impl<'a, Library, Module> From<&'a Library> for Visitors<&'a Library, Module> {
    fn from(value: &'a Library) -> Self {
        Self::Library(value.into())
    }
}
