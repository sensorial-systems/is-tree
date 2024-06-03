### is-tree

Derive everything into a tree.

### Examples


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