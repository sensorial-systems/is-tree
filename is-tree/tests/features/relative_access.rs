// use enum_as_inner::EnumAsInner;
// use is_tree::{AddBranch, HasRelativeAccess, HasValue, HasVisitor, HasVisitorConstructor, IsTree, KnowsValue, RootVisitor, Visitor};

// #[derive(Clone, IsTree, Debug, EnumAsInner)]
// pub enum Visitors<'a> {
//     Root(RootVisitor<&'a Branch>),
//     Branch(Box<Visitor<Visitors<'a>, &'a Branch>>),
//     Leaf(Visitor<Visitors<'a>, &'a Leaf>)
// }

// impl<'a> From<&'a Branch> for Visitors<'a> {
//     fn from(branch: &'a Branch) -> Self {
//         Self::Root(branch.visitor())
//     }
// }

// impl<'a> From<Visitor<Visitors<'a>, &'a Branch>> for Visitors<'a> {
//     fn from(visitor: Visitor<Visitors<'a>, &'a Branch>) -> Self {
//         Self::Branch(visitor.into())
//     }
// }

// impl<'a> From<RootVisitor<&'a Branch>> for Visitors<'a> {
//     fn from(visitor: RootVisitor<&'a Branch>) -> Self {
//         Self::Root(visitor.into())
//     }
// }

// #[derive(IsTree, Debug)]
// #[tree(branches = "Branch")]
// #[tree(visitor = "Visitors<'a>")]
// #[tree(relative_visitor = "Visitors<'a>")]
// pub struct Branch {
//     #[tree(path_segment)]
//     pub name: String,
//     #[tree(branch)]
//     pub branches: Vec<Branch>,
// }

// #[derive(IsTree, Debug)]
// #[tree(visitor = "Visitors<'a>")]
// #[tree(relative_visitor = "Visitors<'a>")]
// pub struct Leaf {
//     #[tree(path_segment)]
//     pub name: String,
//     pub value: i32,
// }

// impl From<String> for Branch {
//     fn from(name: String) -> Self {
//         let branches = Default::default();
//         Self { name, branches }
//     }
// }

// impl<'a> AddBranch<'a> for Branch {
//     fn add_branch(&'a mut self, branch: impl Into<Branch>) -> &'a mut Branch {
//         self.branches.push(branch.into());
//         self.branches.last_mut().unwrap()
//     }
// }

// impl<'a> HasVisitorConstructor<'a> for Visitors<'a> {
//     fn new(parent: Visitors<'a>, value: &'a Branch) -> Visitors<'a> {
//         Visitor::new(parent, value).into()
//     }
// }

// #[test]
// fn relative_access() {
//     let mut branch = Branch::from("grandfather".to_string());
//     branch.add_branch(Branch::from("father".to_string()))
//           .add_branch(Branch::from("son".to_string()));

//     let root = branch.visitor();
//     let son = root.relative(vec!["father", "son"]).unwrap().into_branch().unwrap();
//     assert_eq!(son.as_ref().value().name, "son");
//     assert_eq!(son.relative(vec!["self"]).unwrap().into_branch().unwrap().as_ref().value().name, "son");
//     assert_eq!(son.relative(vec!["super"]).unwrap().into_branch().unwrap().as_ref().value().name, "father");
//     assert_eq!(son.relative(vec!["root"]).unwrap().into_root().unwrap().value().name, "grandfather");
// }