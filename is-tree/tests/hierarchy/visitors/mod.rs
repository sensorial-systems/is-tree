use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner)] // TODO: Use IsTree here
pub enum Visitors<'a, Library, Module> {
    Library(LibraryVisitor<Library>),
    Module(Box<ModuleVisitor<'a, Module>>)
}

impl<'a, Library, Module> KnowsBranches<'a> for Visitors<'a, Library, Module> {
    type Branches = Visitors<'a, Library, Module>;
}

impl<'a> KnowsRoot<'a> for &'a Visitors<'a, &'a Library, &'a Module> {
    type Root = RootVisitor<&'a Library>;
}

impl<'a> KnowsParent<'a> for &'a Visitors<'a, &'a Library, &'a Module> {
    type Parent = Visitors<'a, &'a Library, &'a Module>;
}

impl<'a> HasParent<'a> for &'a Visitors<'a, &'a Library, &'a Module> {
    fn parent(self) -> Self::Parent {
        match self {
            Visitors::Library(value) => (*value).into(),
            Visitors::Module(value) => value.parent().into()
        }
    }

}

impl<'a> HasPathSegment for Visitors<'a, &'a Library, &'a Module> {
    fn path_segment(&self) -> &String {
        match self {
            Visitors::Library(value) => value.path_segment(),
            Visitors::Module(value) => value.path_segment()
        }
    }
}

impl<'a> KnowsBranches<'a> for &'a Visitors<'a, &'a Library, &'a Module> {
    type Branches = Visitors<'a, &'a Library, &'a Module>;
}

impl<'a> HasBranches<'a> for &'a Visitors<'a, &'a Library, &'a Module> {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        match self {
            Visitors::Library(value) => value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(),
            Visitors::Module(value) => value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter()
        }
    }
}

impl<'a> HasGet<'a> for &'a Visitors<'a, &'a Library, &'a Module> {}

impl<'a> HasRoot<'a> for &'a Visitors<'a, &'a Library, &'a Module>
where Self::Root: Clone
{
    fn root(self) -> Self::Root {
        match self {
            Visitors::Library(value) => value.clone(),
            Visitors::Module(value) => value.root()
        }
    }
}

impl<'a, Library, Module> KnowsRelativeAccessType<'a> for Visitors<'a, Library, Module> {
    type RelativeType = Visitors<'a, Library, Module>;
}

impl<'a> HasPath for Visitors<'a, &'a Library, &'a Module> {
    fn path(&self) -> Path {
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
                let value = unsafe { &*(&*value as *const ModuleVisitor<&Module>) };
                value.branches().map(|value| value.into()).collect::<Vec<_>>().into_iter()
            }
        }
    }
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
