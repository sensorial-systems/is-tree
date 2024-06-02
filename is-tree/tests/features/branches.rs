use enum_as_inner::EnumAsInner;
use is_tree::{new_traits::{HasParent, *}, HasPathSegment, HasValue, Visitor};

#[derive(Debug)]
pub struct Library {
    pub name: String,
    pub root_module: Module
}

impl Library {
    pub fn mock() -> Self {
        let name = String::from("greatgrandfather");
        let root_module = Module::mock();
        Self { name, root_module }
    }
}

impl<'a> HasBranches<&'a Module> for &'a Library {
    fn branches_impl(self) -> impl Iterator<Item = &'a Module> {
        std::iter::once(&self.root_module)
    }
}

impl<'a> HasBranches<&'a mut Module> for &'a mut Library {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut Module> {
        std::iter::once(&mut self.root_module)
    }
}

impl AddBranch<Module> for Library {
    fn add_branch(&mut self, module: Module) -> &mut Module {
        self.root_module = module;
        &mut self.root_module
    }
}

impl<'a> HasBranches<&'a String> for &'a Library {
    fn branches_impl(self) -> impl Iterator<Item = &'a String> {
        std::iter::once(&self.name)
            .chain((&self.root_module).branches::<&String>())
    }
}

impl<'a> HasBranches<&'a mut String> for &'a mut Library {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut String> {
        std::iter::once(&mut self.name)
            .chain((&mut self.root_module).branches::<&mut String>())
    }
}

impl AddBranch<String> for Library {
    fn add_branch(&mut self, name: String) -> &mut String {
        self.name = name;
        &mut self.name
    }
}

impl From<&str> for Library {
    fn from(name: &str) -> Self {
        let name = name.into();
        let root_module = Default::default();
        Self { name, root_module }
    }
}

#[derive(Debug, Default)]
pub struct Module {
    pub name: String,
    pub modules: Vec<Module>,
}

impl Module {
    pub fn mock() -> Self {
        let mut branch = Self::from("");
        branch.add_branch(String::from("grandfather"));
        branch.add_branch(Module::from("father"))
            .add_branch(Module::from("son"));
        branch.add_branch(Module::from("uncle"));
        branch
    }
}

impl<'a> HasBranches<&'a Module> for &'a Module {
    fn branches_impl(self) -> impl Iterator<Item = &'a Module> {
        self.modules.iter()
    }
}

impl<'a> HasBranches<&'a mut Module> for &'a mut Module {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut Module> {
        self.modules.iter_mut()
    }
}

impl AddBranch<Module> for Module {
    fn add_branch(&mut self, branch: Module) -> &mut Module {
        self.modules.push(branch);
        self.modules.last_mut().unwrap()
    }
}

impl<'a> HasBranches<&'a String> for &'a Module {
    fn branches_impl(self) -> impl Iterator<Item = &'a String> {
        std::iter::once(&self.name)
            .chain(self.modules.iter().map(|branch| &branch.name))
    }
}

impl<'a> HasBranches<&'a mut String> for &'a mut Module {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut String> {
        std::iter::once(&mut self.name)
            .chain(self.modules.iter_mut().map(|branch| &mut branch.name))
    }
}

impl AddBranch<String> for Module {
    fn add_branch(&mut self, name: String) -> &mut String {
        self.name = name;
        &mut self.name
    }
}

impl From<&str> for Module {
    fn from(name: &str) -> Self {
        let name = name.into();
        let modules = Default::default();
        Self { name, modules }
    }
}

#[test]
fn branches() {
    let mut library = Library::mock();

    (&mut library).branches::<&mut String>().for_each(|s| *s = s.to_uppercase());
    assert_eq!((&library).branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["GREATGRANDFATHER", "GRANDFATHER", "FATHER", "UNCLE"]);
    assert_eq!((&library).branches::<&Module>().map(|module| module.name.as_str()).collect::<Vec<_>>(), vec!["GRANDFATHER"]);

    assert_eq!((&library.root_module).branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["GRANDFATHER", "FATHER", "UNCLE"]);
    assert_eq!((&library.root_module).branches::<&Module>().map(|module| module.name.as_str()).collect::<Vec<_>>(), vec!["FATHER", "UNCLE"]);
}

impl HasPathSegment for Module {
    fn path_segment(&self) -> &String {
        &self.name
    }
}

impl HasPathSegment for Library {
    fn path_segment(&self) -> &String {
        &self.name
    }
}

#[test]
fn get() {
    let mut library = Library::mock();
    assert_eq!((&library).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["grandfather"]);
    if let Some(s) = (&mut library).get::<&mut String>("grandfather") { *s = s.to_uppercase() }
    assert_eq!((&library).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["GRANDFATHER"]);

    assert_eq!((&library.root_module).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["father", "uncle"]);
    if let Some(s) = (&mut library.root_module).get::<&mut String>("father") { *s = s.to_uppercase() }
    assert_eq!((&library.root_module).branches::<&Module>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["FATHER", "uncle"]);
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum Visitors<'a> {
    Root(Visitor<(), &'a Library>),
    Branch(Visitor<Box<Visitors<'a>>, &'a Module>),
}

unsafe impl<'a> UnsafeClone for Visitors<'a> {
    unsafe fn unsafe_clone(&self) -> Self {
        self.clone()
    }
}

unsafe impl<'a> UnsafeBorrow<'a> for Visitors<'a> {
    type Borrow = &'a Visitors<'a>;
    unsafe fn borrow(&'a self) -> Self::Borrow {
        self
    }
}

impl<'a> From<&'a Library> for Visitors<'a> {
    fn from(branch: &'a Library) -> Self {
        Self::Root(Visitor::new((), branch))
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a Module>> for Visitors<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a Module>) -> Self {
        Self::Branch(visitor)
    }
}

impl<'a> HasPathSegment for Visitors<'a> {
    fn path_segment(&self) -> &String {
        match self {
            Visitors::Root(visitor) => visitor.path_segment(),
            Visitors::Branch(visitor) => visitor.path_segment(),
        }
    }
}

impl<'a> HasBranches<Visitors<'a>> for &'a Visitors<'a> {
    fn branches_impl(self) -> impl Iterator<Item = Visitors<'a>> {
        match self {
            Visitors::Root(visitor) => Box::new((*visitor.value()).branches::<&Module>().map(|branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
            Visitors::Branch(visitor) => Box::new((*visitor.value()).branches::<&Module>().map(|branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
        }
    }
}

#[derive(Debug, EnumAsInner)]
pub enum VisitorsMut<'a> {
    Root(Visitor<(), &'a mut Library>),
    Branch(Visitor<Box<Visitors<'a>>, &'a mut Module>),
}

unsafe impl<'a> UnsafeClone for VisitorsMut<'a> {
    unsafe fn unsafe_clone(&self) -> Self {
        let visitor: &Visitors = std::mem::transmute(self);
        let visitor = visitor.clone();
        std::mem::transmute(visitor)
    }
}

unsafe impl<'a> UnsafeBorrow<'a> for VisitorsMut<'a> {
    type Borrow = &'a mut VisitorsMut<'a>;
    unsafe fn borrow(&'a self) -> Self::Borrow {
        #[allow(mutable_transmutes)]
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> From<&'a mut Library> for VisitorsMut<'a> {
    fn from(branch: &'a mut Library) -> Self {
        Self::Root(Visitor::new((), branch))
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a mut Module>> for VisitorsMut<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a mut Module>) -> Self {
        Self::Branch(visitor)
    }
}

impl<'a> From<&&'a mut VisitorsMut<'a>> for Visitors<'a> {
    fn from(visitor: &&'a mut VisitorsMut<'a>) -> Self {
        unsafe {
            (*(std::mem::transmute::<_, &&Visitors<'a>>(visitor))).clone()
        }
    }
}

impl<'a> HasPathSegment for VisitorsMut<'a> {
    fn path_segment(&self) -> &String {
        match self {
            VisitorsMut::Root(visitor) => visitor.path_segment(),
            VisitorsMut::Branch(visitor) => visitor.path_segment(),
        }
    }
}

impl<'a> HasBranches<VisitorsMut<'a>> for &'a mut VisitorsMut<'a> {
    fn branches_impl(self) -> impl Iterator<Item = VisitorsMut<'a>> {
        let parent = Box::new(Visitors::from(&self));
        match self {
            VisitorsMut::Root(visitor) => {
                let parent_clone = parent.clone();
                Box::new((*visitor.value()).branches::<&mut Module>().map(move |branch| Visitor::new(parent_clone.clone(), branch).into())) as Box<dyn Iterator<Item = _>>
            },
            VisitorsMut::Branch(visitor) => {
                let parent_clone = parent.clone();
                Box::new((*visitor.value()).branches::<&mut Module>().map(move |branch| Visitor::new(parent_clone.clone(), branch).into())) as Box<dyn Iterator<Item = _>>
            },
        }
    }
}

#[test]
fn visitor() {
    let mut branch = Library::mock();

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_root().unwrap().value().name, "greatgrandfather");
    assert_eq!((&root_visitor).branches::<Visitors>().map(|visitor| &visitor.as_branch().unwrap().value().name).collect::<Vec<_>>(), vec!["grandfather"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "son", "father", "grandfather", "greatgrandfather"]);

    let mut root_visitor = VisitorsMut::from(&mut branch);
    (&mut root_visitor).branches::<VisitorsMut>().for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Root(visitor) => visitor.value().name = visitor.value().name.to_uppercase(),
            VisitorsMut::Branch(visitor) => visitor.value().name = visitor.value().name.to_uppercase()
        }
    });

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_root().unwrap().value().name, "greatgrandfather");
    assert_eq!((&root_visitor).branches::<Visitors>().map(|visitor| &visitor.as_branch().unwrap().value().name).collect::<Vec<_>>(), vec!["GRANDFATHER"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "son", "father", "GRANDFATHER", "greatgrandfather"]);

    let iterator: TreeIterator<VisitorsMut> = TreeIterator::new(&mut branch);
    iterator.for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Root(visitor) => visitor.value().name = visitor.value().name.to_uppercase(),
            VisitorsMut::Branch(visitor) => visitor.value().name = visitor.value().name.to_uppercase()
        }
    });

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["UNCLE", "SON", "FATHER", "GRANDFATHER", "GREATGRANDFATHER"]);
}

impl<'a> HasParent for Visitors<'a> {
    fn parent(&self) -> Option<Self> {
        match self {
            Visitors::Root(_) => None,
            Visitors::Branch(visitor) => Some((*visitor.parent).clone())
        }
    }
}

unsafe impl<'a> UnsafeHasParent for VisitorsMut<'a> {
    unsafe fn parent_mut(&mut self) -> Option<Self> {
        match self {
            VisitorsMut::Root(_) => None,
            VisitorsMut::Branch(visitor) => {
                let visitor: Visitors = *visitor.parent.clone();
                let visitor = std::mem::transmute(visitor);
                Some(visitor)
            }
        }
    }
}

impl<'a> HasRoot for Visitors<'a> {
    fn root(&self) -> Self {
        match self {
            Visitors::Root(_) => self.clone(),
            Visitors::Branch(visitor) => visitor.parent.root()
        }
    }
}

unsafe impl<'a> UnsafeHasRoot for VisitorsMut<'a> {
    unsafe fn root_mut(&mut self) -> Option<Self> {
        match self {
            VisitorsMut::Root(_) => None,
            VisitorsMut::Branch(visitor) => {
                let visitor: Visitors = visitor.parent.root();
                let visitor = std::mem::transmute(visitor);
                Some(visitor)
            }
        }
    }
}

#[test]
fn relative_access() {
    let branch = Library::mock();

    let greatgrandfather_visitor = Visitors::from(&branch);
    let grandfather_visitor = (&greatgrandfather_visitor).branches::<Visitors>().next().unwrap();
    let father_visitor = (&grandfather_visitor).branches::<Visitors>().next().unwrap();
    let son_visitor = (&father_visitor).branches::<Visitors>().next().unwrap();
    assert_eq!(son_visitor.path_segment(), "son");
    assert_eq!(son_visitor.parent().unwrap().path_segment(), "father");
    assert_eq!(son_visitor.parent().unwrap().parent().unwrap().path_segment(), "grandfather");
    
    assert_eq!(son_visitor.root().path_segment(), "greatgrandfather");
    
    assert_eq!((&grandfather_visitor).get("father").unwrap().path_segment(), "father");

    assert!(greatgrandfather_visitor.relative(vec!["super"]).is_none());
    assert_eq!(greatgrandfather_visitor.relative(Vec::<String>::new()).unwrap().path_segment(), "greatgrandfather");
    assert_eq!(greatgrandfather_visitor.relative(vec!["self"]).unwrap().path_segment(), "greatgrandfather");
    assert_eq!(greatgrandfather_visitor.relative(vec!["root"]).unwrap().path_segment(), "greatgrandfather");
    assert_eq!(greatgrandfather_visitor.relative(vec!["grandfather"]).unwrap().path_segment(), "grandfather");
    assert_eq!(greatgrandfather_visitor.relative(vec!["grandfather", "father", "son"]).unwrap().path_segment(), "son");

    assert_eq!(grandfather_visitor.relative(vec!["super"]).unwrap().path_segment(), "greatgrandfather");

    assert_eq!(son_visitor.relative(vec!["super", "super"]).unwrap().path_segment(), "grandfather");
    assert_eq!(son_visitor.relative(vec!["root"]).unwrap().path_segment(), "greatgrandfather");
}

#[test]
fn unsafe_relative_access() {
    let mut branch = Library::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        let mut father_visitor = (&mut root_visitor).branches::<VisitorsMut>().next().unwrap();
        let mut son_visitor = (&mut father_visitor).branches::<VisitorsMut>().next().unwrap();

        let mut father = son_visitor.parent_mut().unwrap();
        let father = father.as_branch_mut().unwrap();
        father.value().name = father.value().name.to_uppercase();

        let mut grandfather = son_visitor.root_mut().unwrap();
        let grandfather = grandfather.as_root_mut().unwrap();
        grandfather.value().name = grandfather.value().name.to_uppercase();
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "son", "father", "GRANDFATHER", "GREATGRANDFATHER"]);


    let mut branch = Library::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        if let Some(mut visitor) = root_visitor.relative_mut(vec!["grandfather", "father", "son"]) {
            let branch_visitor = visitor.as_branch_mut().unwrap();
            branch_visitor.value().name = branch_visitor.value().name.to_uppercase();

            if let Some(mut visitor) = visitor.relative_mut(vec!["root"]) {
                let visitor = visitor.as_root_mut().unwrap();
                visitor.value().name = visitor.value().name.to_uppercase();
            }
        }
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "SON", "father", "grandfather", "GREATGRANDFATHER"]);

    let mut branch = Library::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        if let Some(mut visitor) = root_visitor.relative_mut(vec!["self"]) {
            let branch_visitor = visitor.as_root_mut().unwrap();
            branch_visitor.value().name = branch_visitor.value().name.to_uppercase();

            if let Some(mut visitor) = visitor.relative_mut(vec!["grandfather", "father", "son", "super"]) {
                let visitor = visitor.as_branch_mut().unwrap();
                visitor.value().name = visitor.value().name.to_uppercase();
            }
        }
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "son", "FATHER", "grandfather", "GREATGRANDFATHER"]);
}