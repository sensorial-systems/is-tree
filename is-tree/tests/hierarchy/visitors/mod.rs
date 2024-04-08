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

impl<'a> HasGet<'a> for &'a Visitors<'a, &'a Library, &'a Module> {}

// TODO: Move it to IsTree derive macro.
impl<'a> HasBranches<'a> for Visitors<'a, &'a Library, &'a Module> {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        match self {
            Self::Library(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&value as *const LibraryVisitor<&Library>) };
                value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter()
            },
            Self::Module(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&*value as *const ModuleVisitor<&Module>) };
                value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter()
            }
        }
    }
}

impl<'a> KnowsRelativeAccessType<'a> for Visitors<'a, &'a Library, &'a Module> {
    type RelativeType = Visitors<'a, &'a Library, &'a Module>;
}

// TODO: Move it to IsTree derive macro.
impl<'a> HasRelativeAccess<'a> for Visitors<'a, &'a Library, &'a Module> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<String>
    {
        match self {
            Self::Library(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&value as *const LibraryVisitor<&Library>) };
                value.relative(path).map(|value| value.into())
            },
            Self::Module(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&*value as *const ModuleVisitor<&Module>) };
                value.relative(path).map(|value| value.into())
            }
        }
    }
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
