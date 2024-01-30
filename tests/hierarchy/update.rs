use super::*;

#[test]
fn add_branch() {
    let mut library = library();
    assert_eq!(library.root_module.branches().count(), 1);
    let branch = Module {
        name: "new".into(),
        children: Default::default()
    };
    assert_eq!(library.root_module.add_branch(branch).name, "new");
    assert_eq!(library.root_module.branches().count(), 2);
}

#[test]
fn get_or_create() {
    let mut library = library();
    assert_eq!(library.root_module.branches().count(), 1);
    assert_eq!(library.root_module.branch("new").name, "new");
    assert_eq!(library.root_module.branches().count(), 2);
    assert_eq!(library.root_module.branch("new").name, "new");
    assert_eq!(library.root_module.branches().count(), 2);
}
