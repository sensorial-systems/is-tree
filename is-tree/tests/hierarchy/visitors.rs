use enum_as_inner::EnumAsInner;

use super::*;

#[derive(Clone, EnumAsInner)]
pub enum Visitors<Library, Module> {
    Library(LibraryVisitor<Library>),
    Module(Box<Visitor<Visitors<Library, Module>, Module>>)
}

// TODO: HasRelativeAccess

impl<'a, Library, Module> ::is_tree::KnowsRelativeAccess<'a> for Visitors<Library, Module>  {
    type RelativeAccess = Visitors<Library, Module>;
}

impl<'a, Library, Module> ::is_tree::HasRelativeAccess<'a> for Visitors<Library, Module>
where
    RootVisitor<Library>: Into<Self::RelativeAccess> + Clone + KnowsRelativeAccess<'a, RelativeAccess = Self> + 'a,
    &'a RootVisitor<Library>: HasValue<'a> + HasParent<'a> + HasRoot<'a> + HasGet<'a>,
    <&'a RootVisitor<Library> as KnowsParent<'a>>::Parent: Into<Self>,
    <&'a RootVisitor<Library> as KnowsRoot<'a>>::Root: Into<Self>,
    <&'a RootVisitor<Library> as KnowsRelativeAccess<'a>>::RelativeAccess: Into<Self>,
    <&'a RootVisitor<Library> as KnowsBranches<'a>>::Branches: Into<Self> + HasPathSegment,

    Visitor<Visitors<Library, Module>, Module>: Into<Self::RelativeAccess> + Clone + KnowsRelativeAccess<'a, RelativeAccess = Self> + 'a,
    &'a Visitor<Visitors<Library, Module>, Module>: HasValue<'a> + HasParent<'a> + HasRoot<'a> + HasGet<'a>,
    <&'a Visitor<Visitors<Library, Module>, Module> as KnowsParent<'a>>::Parent: Into<Self>,
    <&'a Visitor<Visitors<Library, Module>, Module> as KnowsRoot<'a>>::Root: Into<Self>,
    <&'a Visitor<Visitors<Library, Module>, Module> as KnowsRelativeAccess<'a>>::RelativeAccess: Into<Self>,
    <&'a Visitor<Visitors<Library, Module>, Module> as KnowsBranches<'a>>::Branches: Into<Self> + HasPathSegment,
{
    fn relative<K>(self, path: impl IntoIterator<Item = K>) -> Option<Self::RelativeAccess>
    where K: Into<String>
    {
        #[inline]
        fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
        match &self {
            Visitors::Library(visitor) => longer_ref(visitor).relative(path).map(|value| value.into()),
            Visitors::Module(visitor) => longer_ref(visitor).relative(path).map(|value| value.into())
        }
    }
}


// TODO: HasParent

impl<'a, Library, Module> ::is_tree::KnowsParent<'a> for Visitors<Library, Module>{
    type Parent = Visitors<Library, Module>;
}

impl<'a, Library, Module> ::is_tree::HasParent<'a> for &'a Visitors<Library, Module>
where Library: Clone,
      Module: Clone
{
    fn parent(self) -> Self::Parent {
        match self {
            Visitors::Library(visitor) => visitor.parent().into(),
            Visitors::Module(visitor) => visitor.parent().into(),
        }
    }
}

// TODO: HasGet

impl<'a, Library, Module> ::is_tree::HasGet<'a> for Visitors<Library, Module>
where Self: ::is_tree::HasBranches<'a>,
      <Self as ::is_tree::KnowsBranches<'a>>::Branches: ::is_tree::HasPathSegment
{}


// TODO: HasBranches

impl<'a, Library, Module> ::is_tree::KnowsBranches<'a> for Visitors<Library, Module>{
    type Branches = Visitors<Library, Module>;
}

impl<'a, Library, Module> ::is_tree::KnowsBranches<'a> for &'a Visitors<Library, Module> {
    type Branches = Visitors<Library, Module>;
}

impl<'a, Library, Module> ::is_tree::HasBranches<'a> for Visitors<Library, Module>
where
    Library: Clone + HasBranches<'a> + 'a,
    Module: Clone + HasBranches<'a> + 'a,

    Library::Branches: KnowsVisitor<'a>,
    Module::Branches: KnowsVisitor<'a>,

    <RootVisitor<Library> as KnowsBranches<'a>>::Branches: HasVisitorConstructor<'a, Value = Library::Branches>,
    <Visitor<Visitors<Library, Module>, Module> as KnowsBranches<'a>>::Branches: HasVisitorConstructor<'a, Value = Module::Branches>,

    RootVisitor<Library>: Into<<<RootVisitor<Library> as KnowsBranches<'a>>::Branches as KnowsParent<'a>>::Parent> + Clone,
    Visitor<Visitors<Library, Module>, Module>: Into<<<Visitor<Visitors<Library, Module>, Module> as KnowsBranches<'a>>::Branches as KnowsParent<'a>>::Parent> + Clone,

    <<Library as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor: Into<Self::Branches>,
    <<Module as KnowsBranches<'a>>::Branches as KnowsVisitor<'a>>::Visitor: Into<Self::Branches>
{
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
impl<'a, Library, Module> ::is_tree::HasPathSegment for Visitors<Library, Module>
where Library: HasPathSegment + Clone,
      Module: HasPathSegment + Clone,
{
    fn path_segment(&self) -> &String {
        match self {
            Visitors::Library(visitor) => visitor.path_segment(),
            Visitors::Module(visitor) => visitor.path_segment(),
        }
    }
}

impl<'a, Library, Module> ::is_tree::HasPath for Visitors<Library, Module>
where Library: HasPathSegment + Clone,
      Module: HasPathSegment + Clone
{
    fn path(&self) -> ::is_tree::Path {
        match self {
            Visitors::Library(visitor) => visitor.path(),
            Visitors::Module(visitor) => visitor.path(),
        }
    }
}



// TODO: HasRoot
impl<'a, Library, Module> KnowsRoot<'a> for Visitors<Library, Module> {
    type Root = LibraryVisitor<Library>;
}

impl<'a, Library, Module> HasRoot<'a> for &'a Visitors<Library, Module>
where Library: Clone,
      Module: Clone
{
    fn root(self) -> Self::Root {
        match self {
            Visitors::Library(visitor) => visitor.root(),
            Visitors::Module(visitor) => visitor.root(),
        }
    }
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
