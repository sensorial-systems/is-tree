use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner, IsTree)]
pub enum Visitors<'a> {
    Library(LibraryVisitor<&'a Library>),
    Module(ModuleVisitor<'a, &'a Module>)
}

// TODO: Move it to IsTree derive macro.
impl<'a> HasBranches<'a> for Visitors<'a> {
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
impl<'a> HasRelativeAccess<'a> for Visitors<'a> {
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

impl<'a> From<Visitors<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: Visitors<'a>) -> Self {
        match visitor {
            Visitors::Library(library) => Self::Library(library),
            Visitors::Module(module) => Self::Module(module.into())
        }
    }
}

impl<'a> From<ModuleParentVisitor<'a>> for Visitors<'a> {
    fn from(visitor: ModuleParentVisitor<'a>) -> Self {
        match visitor {
            ModuleParentVisitor::Library(library) => Self::Library(library),
            ModuleParentVisitor::Module(module) => Self::Module(*module)
        }
    }
}

impl<'a> From<LibraryVisitor<&'a Library>> for Visitors<'a> {
    fn from(visitor: LibraryVisitor<&'a Library>) -> Self {
        Self::Library(visitor)
    }
}

impl<'a> From<ModuleVisitor<'a, &'a Module>> for Visitors<'a> {
    fn from(visitor: ModuleVisitor<'a, &'a Module>) -> Self {
        Self::Module(visitor)
    }
}
