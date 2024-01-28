use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner, IsTree)]
pub enum Visitors<'a> {
    Library(LibraryVisitor<'a>),
    Module(ModuleVisitor<'a>)
}

// TODO: Move it to IsTree derive macro.
impl<'a> HasBranches<'a> for Visitors<'a> {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        match self {
            Self::Library(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&value as *const LibraryVisitor) };
                value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter()
            },
            Self::Module(value) => {
                // FIXME: This is a workaround.
                let value = unsafe { &*(&value as *const ModuleVisitor) };
                value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter()
            }
        }
    }
}

impl<'a> From<Visitors<'a>> for ModuleParentVisitor<'a> {
    fn from(visitor: Visitors<'a>) -> Self {
        match visitor {
            Visitors::Library(library) => Self::Library(library),
            Visitors::Module(module) => Self::Module(module)
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
