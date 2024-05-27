// use ::is_tree::*;

// #[derive(Default, Debug, IsTree)]
// // #[tree(type_iterator = "String")]
// // #[tree(type_iterator = "u64")]
// #[tree(branches = "Node")]
// #[tree(visitor = "Visitors<'a>")]
// #[tree(relative_visitor = "Visitors<'a>")]
// pub struct Node {
//     #[tree(path_segment)]
//     pub name: String,
//     pub uuid: u64,
//     #[tree(branch)]
//     pub children: Vec<Node>
// }

// impl<'a> AddBranch<'a> for Node {
//     fn add_branch(&'a mut self, branch: impl Into<Node>) -> &'a mut Node {
//         self.children.push(branch.into());
//         self.children.last_mut().unwrap()
//     }
// }


// impl From<String> for Node {
//     fn from(name: String) -> Self {
//         let children = Default::default();
//         Self { name, uuid: 0, children }
//     }
// }

// #[derive(Debug, Clone, IsTree)]
// pub enum Visitors<'a> {
//     Root(RootVisitor<&'a Node>),
//     Branch(Box<Visitor<Visitors<'a>, &'a Node>>)
// }

// impl<'a> From<RootVisitor<&'a Node>> for Visitors<'a> {
//     fn from(visitor: RootVisitor<&'a Node>) -> Self {
//         Self::Root(visitor)
//     }
// }

// impl<'a> From<Visitor<Visitors<'a>, &'a Node>> for Visitors<'a> {
//     fn from(visitor: Visitor<Visitors<'a>, &'a Node>) -> Self {
//         Self::Branch(Box::new(visitor))
//     }
// }

// #[test]
// fn type_iterator() {
//     let node = Node {
//         name: "Root".into(),
//         uuid: 1,
//         children: vec![
//             Node {
//                 name: "Branch1".into(),
//                 uuid: 2,
//                 children: Default::default()
//             },
//             Node {
//                 name: "Branch2".into(),
//                 uuid: 3,
//                 children: vec![
//                     Node {
//                         name: "Leaf1".into(),
//                         uuid: 4,
//                         children: Default::default()
//                     },
//                     Node {
//                         name: "Leaf2".into(),
//                         uuid: 5,
//                         children: Default::default()
//                     }
//                 ]
//             }
//         ]
//     };
//     // assert_eq!((&node).iter_type::<String>().map(|visitor| visitor.value().as_str()).collect::<Vec<_>>(), vec!["Root", "Branch1", "Branch2", "Leaf1", "Leaf2"]);
//     // for uuid in (&node).iter_type::<u64>() {
//     //     println!("Constant uuid: {}", uuid);
//     // }
//     // for string in (&mut node).iter_type::<String>() {
//     //     println!("Mutable string: {}", string);
//     // }
// }
