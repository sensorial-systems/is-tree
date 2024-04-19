use is_tree::{AddBranch, HasBranches, HasGet, HasGetOrCreate, IsTree};

#[derive(IsTree)]
pub struct Branch {
    #[tree(path_segment)]
    pub name: String,
    #[tree(branch)]
    pub branches: Vec<Branch>,
}

impl From<String> for Branch {
    fn from(name: String) -> Self {
        Self {
            name,
            branches: Vec::new(),
        }
    }
}

impl<'a> AddBranch<'a> for Branch {
    fn add_branch(&'a mut self, branch: impl Into<<Self::Branches as is_tree::KnowsOwned>::Owned>) -> &'a mut <Self::Branches as is_tree::KnowsOwned>::Owned
        where Self::Branches: is_tree::KnowsOwned
    {
        self.branches.push(branch.into());
        self.branches.last_mut().unwrap()
    }
}

#[test]
fn branches() {
    let mut branch = Branch::from("root".to_string());
    branch.add_branch(Branch::from("child1".to_string()));
    branch.add_branch(Branch::from("child2".to_string()));
    branch.branch("child3").name = "child3".to_string();
    branch.branch("child").name = "child4".to_string();

    assert_eq!(branch.branches().count(), 4);
    assert_eq!(branch.get("child4").unwrap().name, "child4");
    assert_eq!(branch.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["child1", "child2", "child3", "child4"])
}