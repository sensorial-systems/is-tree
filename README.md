### is-tree

`is-tree` makes everything a tree.

### TODO: Build something (maybe a matrix) for representing which feature an attribute enables. E.g. #[tree(path_segment)] + #[tree(visitor)] enables `RelativeAccess`

### Features

The example structure:
```rust
use ::is_tree::*;

#[derive(IsTree)]
pub struct Node {
    pub name: String,
    pub uuid: u64,
    pub children: Vec<Node>
}

impl Node {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let uuid = Default::default();
        let children = Default::default();
        Self { name, uuid, children }
    }

    pub fn add(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }

    pub fn mock() -> Self {
        Self::new("root")
            .add(Self::new("branch")
                .add(Self::new("leaf")))
    }
}
```

##### Relative access

```rust
fn main() {
    // let node = Node::mock();
    // let leaf = node.get_relative(vec!["branch", "leaf"]).unwrap();
    // assert_eq!(leaf.name, "leaf");
    // let branch = leaf.get_relative("super").unwrap();
    // assert_eq!(branch.name, "branch");
    // assert_eq!(branch.get_relative("root").unwrap().name, leaf.get_relative("root").unwrap().name);
}
```

##### Tree iterator

```rust
// Tree iterator here.
```

##### Multi-type tree

```rust
// Multi-type tree here.
```

##### Type iterator

```rust
use ::is_tree::*;

#[derive(Default, IsTree)]
#[tree(type_iterator = "String")]
#[tree(type_iterator = "u64")]
pub struct Node {
    pub name: String,
    pub uuid: u64,
    pub children: Vec<Node>
}

fn main() {
    // let node = Node::default();
    // for uuid in (&node).iter_type::<u64>() {
    //     println!("Constant uuid: {}", uuid);
    // }
    // for string in (&mut node).iter_type::<String>() {
    //     println!("Mutable string: {}", string);
    // }
}
```

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