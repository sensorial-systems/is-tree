use ::is_tree::*;

pub mod library;
pub mod module;
pub mod visitors;

use library::*;
use module::*;
use visitors::*;

#[test]
fn new_visitor() {
    let library = Library {
        name: String::from("a"),
        root_module: Module {
            name: String::from("b"),
            children: vec![
                Module {
                    name: String::from("c"),
                    children: vec![
                        Module {
                            name: String::from("d"),
                            children: vec![
                                Module {
                                    name: String::from("1"),
                                    children: vec![]
                                },
                                Module {
                                    name: String::from("2"),
                                    children: vec![]
                                },
                                Module {
                                    name: String::from("3"),
                                    children: vec![]
                                }
                            ]
                        }
                    ]
                }
            ]
        }
    };
    let a = &library;
    let b = &a.root_module;
    let c = &b.children[0];
    let d = &c.children[0];
    let a: LibraryVisitor = a.visit();
    let b: ModuleVisitor = a.visit(b);
    let c: ModuleVisitor = b.visit(c);
    let d: ModuleVisitor = c.visit(d);

    assert_eq!(a.path().to_string(), "a");
    assert_eq!(b.path().to_string(), "a::b");
    assert_eq!(c.path().to_string(), "a::b::c");
    assert_eq!(d.path().to_string(), "a::b::c::d");

    let visitors = Visitors::from(a.clone());

    assert_eq!(*visitors.parent().path_segment(), "a");
    assert_eq!(*a.parent().path_segment(), "a"); // Root's parent is itself. Will it create any kind of problem?
    assert_eq!(*b.parent().path_segment(), "a");
    assert_eq!(*c.parent().path_segment(), "b");
    assert_eq!(*d.parent().path_segment(), "c");
    assert_eq!(*c.parent().parent().path_segment(), "a");
    assert_eq!(*d.parent().parent().parent().path_segment(), "a");

    assert_eq!(*visitors.root().path_segment(), "a");
    assert_eq!(*a.root().path_segment(), "a");
    assert_eq!(*b.root().path_segment(), "a");
    assert_eq!(*c.root().path_segment(), "a");
    assert_eq!(*d.root().path_segment(), "a");

    assert_eq!(a.get("b").unwrap().get("c").unwrap().path_segment(), "c");
    assert_eq!(visitors.get("b").unwrap().get("c").unwrap().path_segment(), "c");

    assert_eq!(*a.relative(vec!["super"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*a.relative(vec!["self"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*a.relative(vec!["root"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*a.relative(vec!["b"]).unwrap().as_module().unwrap().path_segment(), "b");
    
    assert_eq!(*b.relative(vec!["self"]).unwrap().as_module() .unwrap().path_segment(), "b");
    assert_eq!(*b.relative(vec!["super"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*b.relative(vec!["root"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*b.relative(vec!["c"]).unwrap().as_module() .unwrap().path_segment(), "c");
    assert_eq!(*c.relative(vec!["super", "super"]).unwrap().as_library().unwrap().path_segment(), "a");
    assert_eq!(*a.relative(vec!["b", "c"]).unwrap().as_module().unwrap().path_segment(), "c");

    assert_eq!(*c.relative(vec!["root", "b", "super", "b", "c", "super", "self"]).unwrap().as_module().unwrap().path_segment(), "b");
}
