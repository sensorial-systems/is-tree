use is_tree::{AddBranch, HasBranches, IsTree};

#[derive(IsTree)]
#[tree(branches = "String")]
pub struct Name {
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

#[test]
fn get_access() {
    // TODO: Support this.
    // let name = Name {
    //     first: "John".to_string(),
    //     middle: Some("Jingleheimer".to_string()),
    //     last: "Doe".to_string(),
    // };

    // assert_eq!(name.get("first"), Some(&"John".to_string()));
    // assert_eq!(name.get("middle"), Some(&"Jingleheimer".to_string()));
    // assert_eq!(name.get("last"), Some(&"Doe".to_string()));
    // assert_eq!(name.get("nickname"), None);
}