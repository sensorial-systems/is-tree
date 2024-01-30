use super::*;

impl<'a> HasGetOrCreate<'a> for Module {
    fn branch<PathSegment>(&mut self, segment: PathSegment) -> &mut Self::Branches
        where Self::Branches: KnowsPathSegment,
              PathSegment: Into<<Self::Branches as KnowsPathSegment>::PathSegment>,
              <Self::Branches as KnowsPathSegment>::PathSegment: Into<Self::Branches>
    {
        let segment = segment.into();
        // This works and it's safe, but the borrow checker doesn't like it.
        // https://rust-lang.github.io/rfcs/2094-nll.html#problem-case-3-conditional-control-flow-across-functions
        let myself = unsafe { &mut *(self as *mut Self) };
        if let Some(branch) = myself.get(segment.clone()) {
            branch
        } else {
            self.add_branch(segment)
        }
    }
}

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
