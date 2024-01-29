use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner)]
pub enum Visitors<'a, Library, Module> {
    Library(LibraryVisitor<Library>),
    Module(ModuleVisitor<'a, Module>)
}

impl<'a, Library, Module> KnowsBranches<'a> for Visitors<'a, Library, Module> {
    type Branches = Visitors<'a, Library, Module>;
}

impl<'a, Library, Module> KnowsRelativeAccessType<'a> for Visitors<'a, Library, Module> {
    type RelativeType = Visitors<'a, Library, Module>;
}

impl<'a, Library, Module> KnowsPathSegment for Visitors<'a, Library, Module> {
    type PathSegment = String;
}

impl<'a> HasPath for Visitors<'a, &'a Library, &'a Module> {
    fn path(&self) -> Path<Self::PathSegment> {
        match self {
            Self::Library(value) => value.path(),
            Self::Module(value) => value.path()
        }
    }
}

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
                let value = unsafe { &*(&value as *const ModuleVisitor<&Module>) };
                value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter()
            }
        }
    }
}

// TODO: Move it to IsTree derive macro.
impl<'a> HasRelativeAccess<'a> for Visitors<'a, &'a Library, &'a Module> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<<Self as KnowsPathSegment>::PathSegment>
    {
        match self {
            Self::Library(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&value as *const LibraryVisitor<&Library>) };
                value.relative(path).map(|value| value.into())
            },
            Self::Module(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&value as *const ModuleVisitor<&Module>) };
                value.relative(path).map(|value| value.into())
            }
        }
    }
}

impl<'a> From<Visitors<'a, &'a Library, &'a Module>> for ModuleParentVisitor<'a> {
    fn from(visitor: Visitors<'a, &'a Library, &'a Module>) -> Self {
        match visitor {
            Visitors::Library(library) => Self::Library(library),
            Visitors::Module(module) => Self::Module(module.into())
        }
    }
}

impl<'a> From<ModuleParentVisitor<'a>> for Visitors<'a, &'a Library, &'a Module> {
    fn from(visitor: ModuleParentVisitor<'a>) -> Self {
        match visitor {
            ModuleParentVisitor::Library(library) => Self::Library(library),
            ModuleParentVisitor::Module(module) => Self::Module(*module)
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
        Self::Module(visitor)
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
