use is_tree::{AddBranch, HasBranches, HasVisitor, IsTree, RootVisitor, Visitor, KnowsRoot};

// #[derive(Clone, IsTree)]
// #[tree(branches = "Visitors<'a>")]
// #[tree(reference = "Visitors<'a>")]
// #[tree(visitor = "Visitors<'a>")]
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


#[derive(IsTree)]
#[tree(branches = "Branch")]
// #[tree(visitor = "Visitors<'a>")]
// #[tree(relative_visitor = "Visitors<'a>")]
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

#[test]
fn branches() {
    let mut branch = Branch::from("root".to_string());
    branch.add_branch(Branch::from("child1".to_string()));
    branch.add_branch(Branch::from("child2".to_string()));

    // TODO: How to support TreeIterator here without more #[tree] attributes?
    // let iterator = is_tree::TreeIterator::new(&branch);

    assert_eq!(branch.branches().count(), 2);
    assert_eq!(branch.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["child1", "child2"])
}