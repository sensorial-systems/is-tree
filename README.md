[![](https://dcbadge.vercel.app/api/server/rzaesS82MT)](https://discord.gg/rzaesS82MT)

### is-tree

Convert everything into a tree structure that supports multi-type visitors for tree iterators and relative access.

### Fully-featured example

```rust
use is_tree::*;

visitor! {
    pub enum Visitors, VisitorsMut {
        Root(Library visits [Module]),
        Branches(
            Module visits [Module, Function],
            Function
        )
    }
}

#[derive(Debug, IsTree)]
pub struct Library {
    #[tree(path_segment)]
    #[tree(branch(String))]
    pub name: String,
    #[tree(branch(Module, String))]
    pub root_module: Module
}

#[derive(Debug, Default, IsTree)]
pub struct Function {
    #[tree(path_segment)]
    #[tree(branch(String))]
    pub name: String
}

#[derive(Debug, Default, IsTree)]
pub struct Module {
    #[tree(path_segment)]
    #[tree(branch(String))]
    pub name: String,
    #[tree(branch(Module, String))]
    pub modules: Vec<Module>,
    #[tree(branch(Function, String))]
    pub functions: Vec<Function>
}

impl Library {
    pub fn mock() -> Self {
        Library {
            name: String::from("library"),
            root_module: Module {
                name: String::from("math"),
                modules: vec![
                    Module {
                        name: String::from("geometry"),
                        modules: vec![Module { name: String::from("shapes"), .. Default::default() }],
                        .. Default::default()
                    },
                    Module {
                        name: String::from("algebra"),
                        functions: vec![Function { name: String::from("exponential") }],
                        .. Default::default()
                    },
                ],
                .. Default::default()
            },
        }
    }
}

fn main() {
    let mut library = Library::mock();
    
    // Getting the String branches of the structure.
    library.branches_mut::<&mut String>().for_each(|s| *s = s.to_uppercase());
    library.branches::<&String>().for_each(|s| println!("{}", s));

    // Getting a Module of the structure.
    library.get_mut::<&mut Module>("MATH").unwrap().name.push_str("EMATICS");
    println!("{}", library.get::<&Module>("MATHEMATICS").unwrap().name);

    // Getting an mutable tree visitor.
    let iterator: TreeIterator<VisitorsMut> = TreeIterator::new(&mut library);
    iterator.for_each(|mut visitor| {
        match &mut visitor {
            VisitorsMut::Library(visitor) => visitor.value.name = visitor.value.name.to_lowercase(),
            VisitorsMut::Module(visitor) => visitor.value.name = visitor.value.name.to_lowercase(),
            VisitorsMut::Function(visitor) => visitor.value.name = visitor.value.name.to_lowercase()
        }
    });

    // Getting a constant tree visitor.
    let iterator: TreeIterator<Visitors> = TreeIterator::new(&library);
    iterator.for_each(|visitor| println!("{}", visitor.path_segment()));

    // Getting the root visitor.
    let root_visitor = Visitors::from(&library);

     // Root don't have a parent.
    assert!(root_visitor.parent().is_none());

    // Root is the root of the structure.
    assert_eq!(root_visitor.root().as_library().unwrap().value.name, "library");

    // "self", "super" amd "root" are special path segments.
    assert_eq!(root_visitor.relative(vec!["self"]).unwrap().path(), Path::from(vec!["library"]));

    // Accessing Module structure.
    assert_eq!(root_visitor.relative(vec!["mathematics"]).unwrap().as_module().unwrap().value.name, "mathematics"); 

    // Using "super".
    assert_eq!(root_visitor.relative(vec!["mathematics", "algebra", "super"]).unwrap().as_module().unwrap().value.name, "mathematics"); 

    // Access a Function structure.
    assert_eq!(root_visitor.relative(vec!["mathematics", "algebra", "exponential"]).unwrap().as_function().unwrap().value.name, "exponential"); 

    // This is allowed.
    assert_eq!(root_visitor.relative(vec!["mathematics", "algebra", "exponential", "root"]).unwrap().as_library().unwrap().value.name, "library"); 

    // Mutably accessing the visitor's parent is unsafe because it allows you to get two mutable references to the same object.
    unsafe {
        let mut root_visitor = VisitorsMut::from(&mut library);

        assert!(root_visitor.parent_mut().is_none());

        root_visitor.root_mut().as_library_mut().unwrap().value.name = "LIBRARY".into();

        root_visitor.relative_mut(vec!["mathematics"]).unwrap().as_module_mut().unwrap().value.name = "MATHEMATICS".into(); 
    }
}
```
