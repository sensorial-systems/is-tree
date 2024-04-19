use is_tree::{AddBranch, HasBranches, IsTree};

#[derive(IsTree)]
pub struct Branch {
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

    // TODO: TreeIterator should be supported here.
    // let iterator = TreeIterator::new(&branch);

    assert_eq!(branch.branches().count(), 2);
    assert_eq!(branch.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["child1", "child2"])
}