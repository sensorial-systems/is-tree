use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner, IsTree)]
#[tree(branches = "Visitors<'a, &'a Library, &'a Module>")]
#[tree(reference = "Visitors<'a, &'a Library, &'a Module>")]
#[tree(visitor = "Visitors<'a, &'a Library, &'a Module>")]
pub enum Visitors<'a, Library, Module> {
    Library(LibraryVisitor<Library>),
    Module(Box<ModuleVisitor<'a, Module>>)
}

impl<'a, Library, Module> KnowsRoot<'a> for Visitors<'a, Library, Module> {
    type Root = LibraryVisitor<Library>;
}

// todo!("CONTINUE HERE");
// impl<'a, Library, Module> ::is_tree::HasRoot<'a> for &'a Visitors<'a, Library, Module>
// where Library: Clone + HasRoot<'a, Root = RootVisitor<Library>>,
//       Module: Clone + HasRoot<'a, Root = RootVisitor<Library>>

// impl<'a, Library, Module> ::is_tree::HasRoot<'a> for &'a Visitors<'a, Library, Module>
impl<'a> HasRoot<'a> for &'a Visitors<'a, &'a Library, &'a Module>
{
    fn root(self) -> Self::Root {
        match self {
            Visitors::Library(visitor) => visitor.root(),
            Visitors::Module(visitor) => visitor.root(),
        }
    }
}

impl<'a, Library, Module> From<LibraryVisitor<Library>> for Visitors<'a, Library, Module> {
    fn from(visitor: LibraryVisitor<Library>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a, Library, Module> From<RootVisitor<&'a mut Library>> for Visitors<'a, &'a Library, Module> {
    fn from(visitor: RootVisitor<&'a mut Library>) -> Self {
        Self::Library(visitor.into())
    }
}

impl<'a, Library, Module> From<ModuleVisitor<'a, Module>> for Visitors<'a, Library, Module> {
    fn from(visitor: ModuleVisitor<'a, Module>) -> Self {
        Self::Module(visitor.into())
    }
}

impl<'a, Library, Module> From<&'a Library> for Visitors<'a, &'a Library, Module> {
    fn from(value: &'a Library) -> Self {
        Self::Library(value.into())
    }
}
