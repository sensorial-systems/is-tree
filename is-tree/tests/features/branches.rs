use enum_as_inner::EnumAsInner;
use is_tree::{new_traits::*, HasPathSegment, HasValue, Visitor};

#[derive(Debug)]
pub struct Branch {
    pub name: String,
    pub branches: Vec<Branch>,
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
    let mut branch = Branch::from("");
    branch.add_branch(String::from("root"));
    branch.add_branch(Branch::from("child1"));
    branch.add_branch(Branch::from("child2"));

    assert_eq!((&branch).branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["root", "child1", "child2"]);
    assert_eq!((&branch).branches::<&Branch>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["child1", "child2"])
}

impl HasPathSegment for Branch {
    fn path_segment(&self) -> &String {
        &self.name
    }
}

#[test]
fn get() {
    let mut branch = Branch::from("");
    branch.add_branch(String::from("root"));
    branch.add_branch(Branch::from("child1"));
    branch.add_branch(Branch::from("child2"));
    assert_eq!((&branch).get::<&Branch>("child1").unwrap().name, "child1");
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum Visitors<'a> {
    Root(Visitor<(), &'a Branch>),
    Branch(Visitor<Box<Visitors<'a>>, &'a Branch>),
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

impl<'a> HasBranches<Visitors<'a>> for Visitors<'a> {
    fn branches_impl(self) -> impl Iterator<Item = Visitors<'a>> {
        std::iter::empty()
        // fn longer_ref<'longer, T>(t: &T) -> &'longer T { unsafe { &*(t as *const T) } }
        // match self {
        //     Visitors::Root(visitor) => visitor.value().branches::<&Branch>().map(|branch| Visitor::new(self.clone().into(), branch).into()),
        //     Visitors::Branch(visitor) => todo!(),
        // }
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

#[test]
fn visitor() {
    let mut branch = Branch::from("grandfather");
    branch.add_branch(Branch::from("father"))
          .add_branch(Branch::from("son"));

    let root_visitor = Visitors::from(&branch);
    assert_eq!(root_visitor.as_root().unwrap().value().name, "grandfather");
    assert_eq!((&root_visitor).branches::<Visitors>().map(|visitor| &visitor.as_branch().unwrap().value().name).collect::<Vec<_>>(), vec!["father"]);

    let iterator: TreeIterator<Visitors> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["grandfather", "father", "son"]);
}
