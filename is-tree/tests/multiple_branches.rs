use is_tree::{AddBranch, HasBranches, IsTree};

#[derive(IsTree)]
#[tree(branches = "String")]
pub struct Name {
    #[tree(path_segment)]
    #[tree(branch)]
    pub first: String,
    #[tree(branch)]
    pub middle: Option<String>,
    #[tree(branch)]
    pub last: String,
}

impl<'a> AddBranch<'a> for Name {
    fn add_branch(&'a mut self, branch: impl Into<String>) -> &'a mut String {
        self.middle = Some(branch.into());
        self.middle.as_mut().unwrap()
    }
}

#[test]
fn multiple_branches() {
    let mut name = Name {
        first: "John".to_string(),
        middle: None,
        last: "Doe".to_string(),
    };

    let branches: Vec<String> = name.branches().cloned().collect();
    assert_eq!(branches, vec!["John", "Doe"]);

    let middle = name.add_branch("Jingleheimer");
    assert_eq!(middle, "Jingleheimer");

    let branches: Vec<String> = name.branches().cloned().collect();
    assert_eq!(branches, vec!["John", "Jingleheimer", "Doe"]);
}