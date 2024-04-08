use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner, IsTree)] // TODO: Use IsTree here
#[tree(branches = "Visitors<'a, &'a Library, &'a Module>")]
#[tree(reference = "Visitors<'a, &'a Library, &'a Module>")]
#[tree(visitor = "Visitors<'a, &'a Library, &'a Module>")]
pub enum Visitors<'a, Library, Module> {
    Library(LibraryVisitor<Library>),
    Module(Box<ModuleVisitor<'a, Module>>)
}

impl<'a> From<LibraryVisitor<&'a Library>> for Visitors<'a, &'a Library, &'a Module> {
    fn from(visitor: LibraryVisitor<&'a Library>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<ModuleVisitor<'a, &'a Module>> for Visitors<'a, &'a Library, &'a Module> {
    fn from(visitor: ModuleVisitor<'a, &'a Module>) -> Self {
        Self::Module(visitor.into())
    }
}

impl<'a> From<RootVisitor<&'a mut Library>> for Visitors<'a, &'a Library, &'a Module> {
    fn from(visitor: RootVisitor<&'a mut Library>) -> Self {
        Self::Library(visitor.into())
    }
}

impl<Library, Module> From<Library> for Visitors<'_, Library, Module> {
    fn from(value: Library) -> Self {
        Self::Library(value.into())
    }
}
