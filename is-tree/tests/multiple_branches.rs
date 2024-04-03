// use is_tree::{AddBranch, IsTree};

// #[derive(IsTree)]
// #[tree(branches = "String")]
// pub struct Name {
//     #[tree(path_segment)]
//     #[tree(branches)]
//     pub first: String,
//     pub middle: Option<String>,
//     pub last: String,
// }

// impl<'a> AddBranch<'a> for Name {
//     fn add_branch(&'a mut self, branch: impl Into<String>) -> &'a mut String {
//         self.middle = Some(branch.into());
//         self.middle.as_mut().unwrap()
//     }
// }