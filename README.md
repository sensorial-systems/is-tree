# WIP - Unstable API

# is-tree
Everything is a tree

### Principles

1. traits should not have references to self to give it more implementation freedom.
```
trait Trait {
    fn function(self);
}

impl Trait for Structure {
    fn function(self) {
        // Self is moved or copied
    }
}

impl Trait for &Structure {
    fn function(self) {
        // Self is taken as a reference
    }
}
```

Use case in is-tree:
```
HasGet
(TODO: Give more details)
```