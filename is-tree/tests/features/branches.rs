use enum_as_inner::EnumAsInner;
use is_tree::{new_traits::{HasParent, *}, HasPathSegment, HasValue, Visitor};

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub branches: Vec<Branch>,
}

impl Branch {
    pub fn mock() -> Self {
        let mut branch = Self::from("");
        branch.add_branch(String::from("grandfather"));
        branch.add_branch(Branch::from("father"))
            .add_branch(Branch::from("son"));
        branch.add_branch(Branch::from("uncle"));
        branch
    }
}

impl<'a> HasBranches<&'a Branch> for &'a Branch {
    fn branches_impl(self) -> impl Iterator<Item = &'a Branch> {
        self.branches.iter()
    }
}

impl<'a> HasBranches<&'a mut Branch> for &'a mut Branch {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut Branch> {
        self.branches.iter_mut()
    }
}

impl AddBranch<Branch> for Branch {
    fn add_branch(&mut self, branch: Branch) -> &mut Branch {
        self.branches.push(branch);
        self.branches.last_mut().unwrap()
    }
}

impl<'a> HasBranches<&'a String> for &'a Branch {
    fn branches_impl(self) -> impl Iterator<Item = &'a String> {
        std::iter::once(&self.name)
            .chain(self.branches.iter().map(|branch| &branch.name))
    }
}

impl<'a> HasBranches<&'a mut String> for &'a mut Branch {
    fn branches_impl(self) -> impl Iterator<Item = &'a mut String> {
        std::iter::once(&mut self.name)
            .chain(self.branches.iter_mut().map(|branch| &mut branch.name))
    }
}

impl AddBranch<String> for Branch {
    fn add_branch(&mut self, name: String) -> &mut String {
        self.name = name;
        &mut self.name
    }
}

impl From<&str> for Branch {
    fn from(name: &str) -> Self {
        let name = name.into();
        let branches = Default::default();
        Self { name, branches }
    }
}

#[test]
fn branches() {
    let mut branch = Branch::mock();

    (&mut branch).branches::<&mut String>().for_each(|s| *s = s.to_uppercase());
    assert_eq!((&branch).branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["GRANDFATHER", "FATHER", "UNCLE"]);
    assert_eq!((&branch).branches::<&Branch>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["FATHER", "UNCLE"])
}

impl HasPathSegment for Branch {
    fn path_segment(&self) -> &String {
        &self.name
    }
}

#[test]
fn get() {
    let mut branch = Branch::mock();
    assert_eq!((&branch).branches::<&Branch>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["father", "uncle"]);
    if let Some(s) = (&mut branch).get::<&mut String>("father") { *s = s.to_uppercase() }
    assert_eq!((&branch).branches::<&Branch>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["FATHER", "uncle"]);
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum Visitors<'a> {
    Root(Visitor<(), &'a Branch>),
    Branch(Visitor<Box<Visitors<'a>>, &'a Branch>),
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

impl<'a> From<&'a Branch> for Visitors<'a> {
    fn from(branch: &'a Branch) -> Self {
        Self::Root(Visitor::new((), branch))
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a Branch>> for Visitors<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a Branch>) -> Self {
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
            Visitors::Root(visitor) => Box::new((*visitor.value()).branches::<&Branch>().map(|branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
            Visitors::Branch(visitor) => Box::new((*visitor.value()).branches::<&Branch>().map(|branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
        }
    }
}

#[derive(Debug, EnumAsInner)]
pub enum VisitorsMut<'a> {
    Root(Visitor<(), &'a mut Branch>),
    Branch(Visitor<Box<Visitors<'a>>, &'a mut Branch>),
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

impl<'a> From<&'a mut Branch> for VisitorsMut<'a> {
    fn from(branch: &'a mut Branch) -> Self {
        Self::Root(Visitor::new((), branch))
    }
}

impl<'a> From<Visitor<Box<Visitors<'a>>, &'a mut Branch>> for VisitorsMut<'a> {
    fn from(visitor: Visitor<Box<Visitors<'a>>, &'a mut Branch>) -> Self {
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
                Box::new((*visitor.value()).branches::<&mut Branch>().map(move |branch| Visitor::new(parent_clone.clone(), branch).into())) as Box<dyn Iterator<Item = _>>
            },
            VisitorsMut::Branch(visitor) => {
                let parent_clone = parent.clone();
                Box::new((*visitor.value()).branches::<&mut Branch>().map(move |branch| Visitor::new(parent_clone.clone(), branch).into())) as Box<dyn Iterator<Item = _>>
            },
        }
    }
}

#[test]
fn visitor() {
    let mut branch = Branch::mock();

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_root().unwrap().value().name, "grandfather");
    assert_eq!((&root_visitor).branches::<Visitors>().map(|visitor| &visitor.as_branch().unwrap().value().name).collect::<Vec<_>>(), vec!["father", "uncle"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "son", "father", "grandfather"]);

    let mut root_visitor = VisitorsMut::from(&mut branch);
    (&mut root_visitor).branches::<VisitorsMut>().for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Root(visitor) => visitor.value().name = visitor.value().name.to_uppercase(),
            VisitorsMut::Branch(visitor) => visitor.value().name = visitor.value().name.to_uppercase()
        }
    });

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_root().unwrap().value().name, "grandfather");
    assert_eq!((&root_visitor).branches::<Visitors>().map(|visitor| &visitor.as_branch().unwrap().value().name).collect::<Vec<_>>(), vec!["FATHER", "UNCLE"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["UNCLE", "son", "FATHER", "grandfather"]);

    let iterator: TreeIterator<VisitorsMut> = TreeIterator::new(&mut branch);
    iterator.for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Root(visitor) => visitor.value().name = visitor.value().name.to_uppercase(),
            VisitorsMut::Branch(visitor) => visitor.value().name = visitor.value().name.to_uppercase()
        }
    });

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["UNCLE", "SON", "FATHER", "GRANDFATHER"]);
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
    let branch = Branch::mock();

    let root_visitor = Visitors::from(&branch);
    let father_visitor = (&root_visitor).branches::<Visitors>().next().unwrap();
    let son_visitor = (&father_visitor).branches::<Visitors>().next().unwrap();
    assert_eq!(son_visitor.path_segment(), "son");
    assert_eq!(son_visitor.parent().unwrap().path_segment(), "father");
    assert_eq!(son_visitor.parent().unwrap().parent().unwrap().path_segment(), "grandfather");
    
    assert_eq!(son_visitor.root().path_segment(), "grandfather");
    
    assert_eq!((&father_visitor).get("son").unwrap().path_segment(), "son");

    assert!(root_visitor.relative(vec!["super"]).is_none());
    assert_eq!(root_visitor.relative(Vec::<String>::new()).unwrap().path_segment(), "grandfather");
    assert_eq!(root_visitor.relative(vec!["self"]).unwrap().path_segment(), "grandfather");
    assert_eq!(root_visitor.relative(vec!["root"]).unwrap().path_segment(), "grandfather");
    assert_eq!(root_visitor.relative(vec!["father"]).unwrap().path_segment(), "father");
    assert_eq!(root_visitor.relative(vec!["father", "son"]).unwrap().path_segment(), "son");

    assert_eq!(father_visitor.relative(vec!["super"]).unwrap().path_segment(), "grandfather");

    assert_eq!(son_visitor.relative(vec!["super", "super"]).unwrap().path_segment(), "grandfather");
    assert_eq!(son_visitor.relative(vec!["root"]).unwrap().path_segment(), "grandfather");
}

#[test]
fn unsafe_relative_access() {
    let mut branch = Branch::mock();

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
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "son", "FATHER", "GRANDFATHER"]);


    let mut branch = Branch::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        if let Some(mut visitor) = root_visitor.relative_mut(vec!["father", "son"]) {
            let branch_visitor = visitor.as_branch_mut().unwrap();
            branch_visitor.value().name = branch_visitor.value().name.to_uppercase();

            if let Some(mut visitor) = visitor.relative_mut(vec!["root"]) {
                let visitor = visitor.as_root_mut().unwrap();
                visitor.value().name = visitor.value().name.to_uppercase();
            }
        }
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "SON", "father", "GRANDFATHER"]);

    let mut branch = Branch::mock();

    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut branch);
        if let Some(mut visitor) = root_visitor.relative_mut(vec!["self"]) {
            let branch_visitor = visitor.as_root_mut().unwrap();
            branch_visitor.value().name = branch_visitor.value().name.to_uppercase();

            if let Some(mut visitor) = visitor.relative_mut(vec!["father", "son", "super"]) {
                let visitor = visitor.as_branch_mut().unwrap();
                visitor.value().name = visitor.value().name.to_uppercase();
            }
        }
    }

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["uncle", "son", "FATHER", "GRANDFATHER"]);
}