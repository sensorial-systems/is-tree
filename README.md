### is-tree

`is-tree` makes everything a tree.

### TODO: Build something (maybe a matrix) for representing which feature an attribute enables. E.g. #[tree(path_segment)] + #[tree(visitor)] enables `RelativeAccess`

### Features

The example structure:
```rust
#[derive(IsTree)]
pub struct Node {
    pub name: String,
    pub uuid: uuid::Uuid,
    pub children: Vec<Node>
}

impl Node {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let uuid = uuid::new_v4();
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
    let node = Node::mock();
    let leaf = node.get_relative(vec!["branch", "leaf"]).unwrap();
    assert_eq!(leaf.name, "leaf");
    let branch = leaf.get_relative("super").unwrap();
    assert_eq!(branch.name, "branch");
    assert_eq!(branch.get_relative("root").unwrap().name, leaf.get_relative("root").unwrap().name);
}
```

##### Tree iterator

```rust
Tree iterator here.
```

##### Multi-type tree

```rust
Multi-type tree here.
```

##### Type iterator

```rust
#[derive(IsTree)]
#[tree(type_iterator = "String")]
#[tree(type_iterator = "uuid::Uuid")]
pub struct Node {
    pub name: String,
    pub uuid: uuid::Uuid,
    pub children: Vec<Node>
}

fn main() {
    let node = Node::mock();
    for uuid in (&node).iter_type::<uuid::Uuid>() {
        println!("Constant uuid: {}", uuid);
    }
    for string in (&mut node).iter_type::<String>() {
        println!("Mutable string: {}", string);
    }
}
```
