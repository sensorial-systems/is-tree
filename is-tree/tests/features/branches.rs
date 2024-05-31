use enum_as_inner::EnumAsInner;
use is_tree::{new_traits::{HasParent, *}, HasPathSegment, HasValue, Visitor};

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
    let mut branch = Branch::from("");
    branch.add_branch(String::from("root"));
    branch.add_branch(Branch::from("child1"));
    branch.add_branch(Branch::from("child2"));

    (&mut branch).branches::<&mut String>().for_each(|s| *s = s.to_uppercase());
    assert_eq!((&branch).branches::<&String>().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["ROOT", "CHILD1", "CHILD2"]);
    assert_eq!((&branch).branches::<&Branch>().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["CHILD1", "CHILD2"])
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
    if let Some(s) = (&mut branch).get::<&mut String>("child1") { *s = s.to_uppercase() }
    assert_eq!((&branch).get::<&Branch>("CHILD1").unwrap().name, "CHILD1");
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum Visitors<Branch> {
    Root(Visitor<(), Branch>),
    Branch(Visitor<Box<Visitors<Branch>>, Branch>),
}

impl<Branch> From<Branch> for Visitors<Branch> {
    fn from(branch: Branch) -> Self {
        Self::Root(Visitor::new((), branch))
    }
}

impl<Branch> From<Visitor<Box<Visitors<Branch>>, Branch>> for Visitors<Branch> {
    fn from(visitor: Visitor<Box<Visitors<Branch>>, Branch>) -> Self {
        Self::Branch(visitor)
    }
}

impl<Branch: Clone + HasPathSegment> HasPathSegment for Visitors<Branch> {
    fn path_segment(&self) -> &String {
        match self {
            Visitors::Root(visitor) => visitor.path_segment(),
            Visitors::Branch(visitor) => visitor.path_segment(),
        }
    }
}

// impl<Branch: HasBranches<Branch> + Clone> HasBranches<Visitors<Branch>> for Visitors<Branch> {
//     fn branches_impl(self) -> impl Iterator<Item = Visitors<Branch>> {
//         match &self {
//             Visitors::Root(visitor) => Box::new((*visitor.value()).branches::<Branch>().map(move |branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
//             Visitors::Branch(visitor) => Box::new((*visitor.value()).branches::<Branch>().map(move |branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
//         }
//     }
// }

impl<'a, Branch: HasBranches<Branch> + Clone> HasBranches<Visitors<Branch>> for &'a Visitors<Branch> {
    fn branches_impl(self) -> impl Iterator<Item = Visitors<Branch>> {
        match self {
            Visitors::Root(visitor) => Box::new((*visitor.value()).clone().branches::<Branch>().map(|branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
            Visitors::Branch(visitor) => Box::new((*visitor.value()).clone().branches::<Branch>().map(|branch| Visitor::new(self.clone().into(), branch).into())) as Box<dyn Iterator<Item = _>>,
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
    assert_eq!((&root_visitor).branches::<Visitors<&Branch>>().map(|visitor| &visitor.as_branch().unwrap().value().name).collect::<Vec<_>>(), vec!["father"]);

    // let iterator: TreeIterator<Visitors<&mut Branch>> = TreeIterator::mutable(&mut branch); // This is broken. The rest is working.

    let iterator: TreeIterator<Visitors<&Branch>> = TreeIterator::constant(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path_segment().clone()).collect::<Vec<_>>(), vec!["son", "father", "grandfather"]);
}

impl<Branch: Clone> HasParent for Visitors<Branch> {
    fn parent(&self) -> Option<Self> {
        match self {
            Visitors::Root(_) => None,
            Visitors::Branch(visitor) => Some((*visitor.parent).clone())
        }
    }
}

impl<Branch: Clone> HasRoot for Visitors<Branch> {
    fn root(&self) -> Self {
        match self {
            Visitors::Root(_) => self.clone(),
            Visitors::Branch(visitor) => visitor.parent.root()
        }
    }
}

#[test]
fn relative_access() {
    let mut branch = Branch::from("grandfather");
    branch.add_branch(Branch::from("father"))
          .add_branch(Branch::from("son"));

    let root_visitor = Visitors::from(&branch);
    let father_visitor = (&root_visitor).branches::<Visitors<&Branch>>().next().unwrap();
    let son_visitor = (&father_visitor).branches::<Visitors<&Branch>>().next().unwrap();
    assert_eq!(son_visitor.path_segment(), "son");
    assert_eq!(son_visitor.parent().unwrap().path_segment(), "father");
    assert_eq!(son_visitor.parent().unwrap().parent().unwrap().path_segment(), "grandfather");
    
    assert_eq!(son_visitor.root().path_segment(), "grandfather");
    
    assert_eq!((&father_visitor).get("son").unwrap().path_segment(), "son");

    // assert!((&root_visitor).relative(vec!["super"]).is_none());
    // assert_eq!(root_visitor.relative(Vec::<String>::new()).unwrap().path_segment(), "grandfather");
    // assert_eq!(root_visitor.relative(vec!["self"]).unwrap().path_segment(), "grandfather");
    // assert_eq!(root_visitor.relative(vec!["root"]).unwrap().path_segment(), "grandfather");
    // assert_eq!(root_visitor.relative(vec!["father"]).unwrap().path_segment(), "father");
    // assert_eq!(root_visitor.relative(vec!["father", "son"]).unwrap().path_segment(), "son");

    // assert_eq!(father_visitor.relative(vec!["super"]).unwrap().path_segment(), "grandfather");

    // assert_eq!(son_visitor.relative(vec!["super", "super"]).unwrap().path_segment(), "grandfather");
    // assert_eq!(son_visitor.relative(vec!["root"]).unwrap().path_segment(), "grandfather");
}