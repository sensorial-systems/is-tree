use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner)]
// #[tree(branches = "Visitors<'a, &'a Library, &'a Module>")]
// #[tree(reference = "Visitors<'a, &'a Library, &'a Module>")]
// #[tree(visitor = "Visitors<'a, &'a Library, &'a Module>")]
pub enum Visitors<'a, Library, Module> {
    Library(LibraryVisitor<Library>),
    Module(Box<ModuleVisitor<'a, Module>>)
}

// TODO: HasRelativeAccess

impl<'a> ::is_tree::KnowsRelativeAccessType<'a> for Visitors<'a, &'a Library, &'a Module>  {
    type RelativeType = Visitors<'a, &'a Library, &'a Module>;
}

impl<'a> ::is_tree::HasRelativeAccess<'a> for Visitors<'a, &'a Library, &'a Module> {
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeType>
    where K: Into<String>
    {
        #[inline]
        fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
        match &self {
            Visitors::Library(visitor) => longer_ref(visitor).relative(path).map(|value| value.into()),
            Visitors::Module(visitor) => longer_ref(visitor).relative(path).map(|value| value.into()),
        }
    }
}


// TODO: HasParent

impl<'a> ::is_tree::KnowsParent<'a> for Visitors<'a, &'a Library, &'a Module>{
    type Parent = Visitors<'a, &'a Library, &'a Module>;
}

impl<'a> ::is_tree::HasParent<'a> for &'a Visitors<'a, &'a Library, &'a Module>{
    fn parent(self) -> Self::Parent {
        match self {
            Visitors::Library(visitor) => visitor.parent().into(),
            Visitors::Module(visitor) => visitor.parent().into(),
        }
    }
}

// TODO: HasGet

impl<'a, Library, Module> ::is_tree::HasGet<'a> for Visitors<'a, Library, Module>
where Self: ::is_tree::HasBranches<'a>,
      <Self as ::is_tree::KnowsBranches<'a>>::Branches: ::is_tree::HasPathSegment
{}


// TODO: HasBranches

impl<'a, Library, Module> ::is_tree::KnowsBranches<'a> for Visitors<'a, Library, Module>{
    type Branches = Visitors<'a, Library, Module>;
}

impl<'a, Library, Module> ::is_tree::KnowsBranches<'a> for &'a Visitors<'a, Library, Module> {
    type Branches = Visitors<'a, Library, Module>;
}

impl<'a> ::is_tree::HasBranches<'a> for &'a Visitors<'a, &'a Library, &'a Module>{
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        fn longer_ref<'longer, T>(t: &T) -> &T { t }
        match self {
            Visitors::Library(visitor) => longer_ref(visitor).branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
            Visitors::Module(visitor) => longer_ref(visitor).branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
        }
    }
}

impl<'a> ::is_tree::HasBranches<'a> for Visitors<'a, &'a Library, &'a Module> {
    fn branches(self) -> impl Iterator<Item = Self::Branches> {
        #[inline]
        fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
        match &self {
            Visitors::Library(visitor) => longer_ref(visitor).branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
            Visitors::Module(visitor) => longer_ref(visitor).branches().map(|value| value.into()).collect::<Vec<_>>().into_iter(), // TODO: This needs optimization.
        }
    }
}


// TODO: HasPath
impl<'a, Library, Module> ::is_tree::HasPathSegment for Visitors<'a, &'a Library, &'a Module>
where Library: HasPathSegment,
      Module: HasPathSegment
{
    fn path_segment(&self) -> &String {
        match self {
            Visitors::Library(visitor) => visitor.path_segment(),
            Visitors::Module(visitor) => visitor.path_segment(),
        }
    }
}

impl<'a> ::is_tree::HasPath for Visitors<'a, &'a Library, &'a Module>{
    fn path(&self) -> ::is_tree::Path {
        match self {
            Visitors::Library(visitor) => visitor.path(),
            Visitors::Module(visitor) => visitor.path(),
        }
    }
}



// TODO: HasRoot
impl<'a, Library, Module> KnowsRoot<'a> for Visitors<'a, Library, Module> {
    type Root = LibraryVisitor<Library>;
}

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
