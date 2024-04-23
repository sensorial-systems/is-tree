use is_tree::{AddBranch, HasPath, HasVisitor, HasVisitorConstructor, IsTree, KnowsValue, RootVisitor, TreeIterator, Visitor};

// #[derive(Clone, IsTree)]
#[derive(Clone, IsTree, Debug)]
#[tree(branches = "Visitors<'a>")]
#[tree(reference = "Visitors<'a>")]
#[tree(visitor = "Visitors<'a>")]
pub enum Visitors<'a> {
    Root(RootVisitor<&'a Branch>),
    Branch(Box<Visitor<Visitors<'a>, &'a Branch>>),
}

impl<'a> From<&'a Branch> for Visitors<'a> {
    fn from(branch: &'a Branch) -> Self {
        Self::Root(branch.visitor())
    }
}

impl<'a> From<Visitor<Visitors<'a>, &'a Branch>> for Visitors<'a> {
    fn from(visitor: Visitor<Visitors<'a>, &'a Branch>) -> Self {
        Self::Branch(visitor.into())
    }
}

impl<'a> From<RootVisitor<&'a Branch>> for Visitors<'a> {
    fn from(visitor: RootVisitor<&'a Branch>) -> Self {
        Self::Root(visitor.into())
    }
}

#[derive(IsTree, Debug)]
#[tree(branches = "Branch")]
#[tree(visitor = "Visitors<'a>")]
#[tree(relative_visitor = "Visitors<'a>")]
pub struct Branch {
    #[tree(path_segment)]
    pub name: String,
    #[tree(branch)]
    pub branches: Vec<Branch>,
}

impl From<String> for Branch {
    fn from(name: String) -> Self {
        let branches = Default::default();
        Self { name, branches }
    }
}

impl<'a> AddBranch<'a> for Branch {
    fn add_branch(&'a mut self, branch: impl Into<Branch>) -> &'a mut Branch {
        self.branches.push(branch.into());
        self.branches.last_mut().unwrap()
    }
}

impl<'a> HasVisitorConstructor<'a> for Visitors<'a> {
    fn new(parent: Visitors<'a>, value: &'a Branch) -> Visitors<'a> {
        Visitor::new(parent, value).into()
    }
}

impl<'a> KnowsValue<'a> for Visitors<'a> {
    type Value = &'a Branch;
}

#[test]
fn visitors() {
    let mut branch = Branch::from("grandfather".to_string());
    branch.add_branch(Branch::from("father".to_string()))
          .add_branch(Branch::from("son".to_string()));

    let iterator: TreeIterator<Visitors<'_>> = TreeIterator::new(&branch);
    assert_eq!(iterator.map(|visitor| visitor.path().to_string()).collect::<Vec<_>>(), vec!["grandfather::father::son", "grandfather::father", "grandfather"]);
}