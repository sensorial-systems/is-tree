use super::*;

#[test]
fn direct_get() {
    let library = library();
    assert_eq!(library.get("b").unwrap().get("c").unwrap().path_segment(), "c");
}

#[test]
fn direct_branches() {
    let library = library();
    let module = &library.root_module.children[0].children[0];

    assert_eq!(library.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["b"]);
    assert_eq!(module.branches().map(|branch| branch.name.as_str()).collect::<Vec<_>>(), vec!["1", "2", "3"]);
}

#[test]
fn visitor_value_and_parent() {
    let library = library();

    let library: LibraryVisitor<&Library> = library.visitor();
    let root_module: ModuleVisitor<&Module> = library.visit(&library.value().root_module);
    let sub_module: ModuleVisitor<&Module> = root_module.visit(&(&root_module).value().children[0]);

    assert_eq!(library.value().path_segment(), "a");
    assert_eq!(root_module.value().path_segment(), "b");
    assert_eq!(library.parent().path_segment(), "a");
    assert_eq!(sub_module.parent().parent().path_segment(), "a");
}

#[test]
fn visitor_get() {
    let library = library();
    let module = &library.root_module;

    let library: LibraryVisitor<&Library> = library.visitor();
    let module: ModuleVisitor<&Module> = library.visit(module);

    assert_eq!(library.get("b").unwrap().get("c").unwrap().path_segment(), "c");
    assert_eq!(module.parent().get("b").unwrap().path_segment(), "b");
}

#[test]
fn visitor_branches() {
    let library = library();
    let library = library.visitor();
    let root_module = library.visit(&library.value().root_module);
    let sub_module = root_module.visit(&(&root_module).value().children[0]);
    let sub_module = sub_module.visit(&(&sub_module).value().children[0]);

    assert_eq!(library.branches().map(|branch| branch.path().to_string()).collect::<Vec<_>>(), vec!["a::b"]);
    assert_eq!(root_module.parent().branches().map(|branch| branch.path().to_string()).collect::<Vec<_>>(), vec!["a::b"]);
    assert_eq!(sub_module.branches().map(|branch| branch.path().to_string()).collect::<Vec<_>>(), vec!["a::b::c::d::1", "a::b::c::d::2", "a::b::c::d::3"]);
}

#[test]
fn visitor_root() {
    let library = library();
    let library = library.visitor();
    let root_module = library.visit(&library.value().root_module);
    let sub_module = root_module.visit(&(&root_module).value().children[0]);
    let sub_module = sub_module.visit(&(&sub_module).value().children[0]);

    assert_eq!(*library.root().path_segment(), "a");
    assert_eq!(*root_module.root().path_segment(), "a");
    assert_eq!(*sub_module.root().path_segment(), "a");
}

#[test]
fn visitor_relative() {
    let library = library();
    let a = library;
    let b = &a.root_module;
    let c = &b.children[0];

    let a: LibraryVisitor<&Library> = a.visitor();
    let b: ModuleVisitor<&Module> = a.visit(b);
    let c: ModuleVisitor<&Module> = b.visit(c);

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
