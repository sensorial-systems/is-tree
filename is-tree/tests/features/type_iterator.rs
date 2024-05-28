use ::is_tree::*;

#[derive(Default, Debug, IsTree)]
#[tree(branches = "Node")]
#[tree(visitor = "Visitors<'a>")]
#[tree(relative_visitor = "Visitors<'a>")]
pub struct Node {
    #[tree(path_segment)]
    #[tree(type_iterator = "String")]
    pub name: String,
    #[tree(type_iterator = "u64")]
    pub uuid: u64,
    #[tree(branch)]
    #[tree(type_iterator = "String")]
    #[tree(type_iterator = "u64")]
    pub children: Vec<Node>
}

impl Node {
    pub fn new(name: impl Into<String>, uuid: u64) -> Self {
        let name = name.into();
        let children = Default::default();
        Self { name, uuid, children }
    }
}

impl<'a> AddBranch<'a> for Node {
    fn add_branch(&'a mut self, branch: impl Into<Node>) -> &'a mut Node {
        self.children.push(branch.into());
        self.children.last_mut().unwrap()
    }
}


impl From<String> for Node {
    fn from(name: String) -> Self {
        let children = Default::default();
        Self {
            name,
            uuid: 0,
            children
        }
    }
}

#[derive(Debug, Clone, IsTree)]
pub enum Visitors<'a> {
    Root(RootVisitor<&'a Node>),
    Branch(Box<Visitor<Visitors<'a>, &'a Node>>)
}

impl<'a> From<RootVisitor<&'a Node>> for Visitors<'a> {
    fn from(visitor: RootVisitor<&'a Node>) -> Self {
        Self::Root(visitor)
    }
}

impl<'a> From<Visitor<Visitors<'a>, &'a Node>> for Visitors<'a> {
    fn from(visitor: Visitor<Visitors<'a>, &'a Node>) -> Self {
        Self::Branch(Box::new(visitor))
    }
}

#[test]
fn type_iterator() {
    let mut node = Node::new("Root", 1);
    node
        .add_branch(Node::new("Branch1", 2))
        .add_branch(Node::new("Leaf1", 3));
    node
        .add_branch(Node::new("Branch2", 4))
        .add_branch(Node::new("Leaf2", 5));
assert_eq!((&node).iter_type::<String>().map(|visitor| visitor.value().as_str()).collect::<Vec<_>>(), vec!["Branch2", "Leaf2", "Branch1", "Leaf1", "Root"]);
    assert_eq!((&node).iter_type::<u64>().map(|visitor| *visitor.value()).collect::<Vec<_>>(), vec![4, 5, 2, 3, 1]);
}
