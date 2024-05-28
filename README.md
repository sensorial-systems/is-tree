### is-tree

Derive everything into a tree.

### TODO: Build something (maybe a matrix) for representing which feature an attribute enables. E.g. #[tree(path_segment)] + #[tree(visitor)] enables `RelativeAccess`

### Examples

<!-- BRANCHES -->
<!-- BRANCHES -->
<!-- BRANCHES -->

<details>
<summary>
Branches
</summary>

```rust
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

    assert_eq!(branch.branches().count(), 2);
    assert_eq!(branch.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["child1", "child2"])
}
```

</details>

<!-- GET ACCESS -->
<!-- GET ACCESS -->
<!-- GET ACCESS -->

<details>
<summary>
Get access
</summary>

```rust
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
        let branches = Default::default();
        Self { name, branches }
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
```
</details>

<!-- TYPE ITERATOR -->
<!-- TYPE ITERATOR -->
<!-- TYPE ITERATOR -->

<details>
<summary>
Type iterator
</summary>

```rust
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
```
</details>

<details>
<summary>
Tree Visitor
</summary>

```rust
use is_tree::{AddBranch, HasPath, HasVisitor, HasVisitorConstructor, IsTree, KnowsValue, RootVisitor, TreeIterator, Visitor};

#[derive(Clone, IsTree, Debug)]
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
```

</details>

<!-- RELATIVE ACCESS -->
<!-- RELATIVE ACCESS -->
<!-- RELATIVE ACCESS -->

<details>
<summary>
Relative access
</summary>

```rust
use enum_as_inner::EnumAsInner;
use is_tree::{AddBranch, HasRelativeAccess, HasValue, HasVisitor, HasVisitorConstructor, IsTree, KnowsValue, RootVisitor, Visitor};

#[derive(Clone, IsTree, Debug, EnumAsInner)]
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
fn relative_access() {
    let mut branch = Branch::from("grandfather".to_string());
    branch.add_branch(Branch::from("father".to_string()))
          .add_branch(Branch::from("son".to_string()));

    let root = branch.visitor();
    let son = root.relative(vec!["father", "son"]).unwrap().into_branch().unwrap();
    assert_eq!(son.as_ref().value().name, "son");
    assert_eq!(son.relative(vec!["self"]).unwrap().into_branch().unwrap().as_ref().value().name, "son");
    assert_eq!(son.relative(vec!["super"]).unwrap().into_branch().unwrap().as_ref().value().name, "father");
    assert_eq!(son.relative(vec!["root"]).unwrap().into_root().unwrap().value().name, "grandfather");
}
```
</details>

##### Multi-type tree

```rust
// Multi-type tree here.
```

---

# Development guide

### Increasing derive coverage

If you don't want to break a working derive, like:
```rust,ignore
#[derive(Clone, IsTree, Debug)]
pub enum Visitors<'a> {
    Root(RootVisitor<&'a Branch>),
    Branch(Box<Visitor<Visitors<'a>, &'a Branch>>),
}
```

And a new case isn't supported yet, then use a #[tree(dev)] flag like:
```rust,ignore
use ::is_tree::*;

#[derive(Clone, IsTree, EnumAsInner)]
#[tree(dev)] // WIP: Supporting generics
pub enum Visitors<Library, Module> {
    Library(RootVisitor<Library>),
    Module(Box<Visitor<Visitors<Library, Module>, Module>>)
}
```

Then you can work on it in the derive function, e.g.:

```rust,ignore
pub fn derive(enumeration: &Enumeration) -> TokenStream {
    if enumeration.has_attribute(vec!["tree", "dev"]) {
        todo!("New implementation for enum with generics")
    } else {
        todo!("Working implementation for enum without generics")
    }
}
```

Once you get it working, try to generalize both cases and get rid of the conditional execution.