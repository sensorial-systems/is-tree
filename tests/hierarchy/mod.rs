use ::is_tree::*;

pub mod library;
pub mod module;
pub mod visitors;

use library::*;
use module::*;

fn hierarchy() -> Library {
    Library {
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
    }
}

#[test]
fn direct_get() {
    let library = hierarchy();
    assert_eq!(library.get("b").unwrap().get("c").unwrap().path_segment(), "c");
}

#[test]
fn direct_branches() {
    let library = hierarchy();
    let module = &library.root_module.children[0].children[0];

    assert_eq!(library.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["b"]);
    assert_eq!(module.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["1", "2", "3"]);
}

#[test]
fn visitor_value_and_parent() {
    let library = hierarchy();

    let library: LibraryVisitor = library.visitor();
    let root_module: ModuleVisitor = library.visit(&library.value().root_module);
    let sub_module: ModuleVisitor = root_module.visit(&root_module.value().children[0]);

    assert_eq!(library.value().path_segment(), "a");
    assert_eq!(root_module.value().path_segment(), "b");
    assert_eq!(library.parent().path_segment(), "a");
    assert_eq!(sub_module.parent().parent().path_segment(), "a");
}

#[test]
fn visitor_get() {
    let library = hierarchy();
    let module = &library.root_module;

    let library: LibraryVisitor = library.visitor();
    let module: ModuleVisitor = library.visit(module);

    assert_eq!(library.get("b").unwrap().get("c").unwrap().path_segment(), "c");
    assert_eq!(module.parent().get("b").unwrap().path_segment(), "b");
}

#[test]
fn visitor_branches() {
    let library = hierarchy();
    let library = library.visitor();
    let root_module = library.visit(&library.value().root_module);
    let sub_module = root_module.visit(&root_module.value().children[0]);
    let sub_module = sub_module.visit(&sub_module.value().children[0]);

    assert_eq!(library.branches().map(|branch| branch.path().to_string()).collect::<Vec<_>>(), vec!["a::b"]);
    assert_eq!(root_module.parent().branches().map(|branch| branch.path().to_string()).collect::<Vec<_>>(), vec!["a::b"]);
    assert_eq!(sub_module.branches().map(|branch| branch.path().to_string()).collect::<Vec<_>>(), vec!["a::b::c::d::1", "a::b::c::d::2", "a::b::c::d::3"]);
}

#[test]
fn visitor_root() {
    let library = hierarchy();
    let library = library.visitor();
    let root_module = library.visit(&library.value().root_module);
    let sub_module = root_module.visit(&root_module.value().children[0]);
    let sub_module = sub_module.visit(&sub_module.value().children[0]);

    assert_eq!(*library.root().path_segment(), "a");
    assert_eq!(*root_module.root().path_segment(), "a");
    assert_eq!(*sub_module.root().path_segment(), "a");
}

#[test]
fn visitor_relative() {
    let library = hierarchy();
    let a = library;
    let b = &a.root_module;
    let c = &b.children[0];

    let a: LibraryVisitor = a.visitor();
    let b: ModuleVisitor = a.visit(b);
    let c: ModuleVisitor = b.visit(c);

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
