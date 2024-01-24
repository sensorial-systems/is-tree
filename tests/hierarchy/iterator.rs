use super::*;

#[test]
fn test() {
    let hierarchy = hierarchy();
    let iterator: TreeVisitor<Visitors<'_>> = TreeVisitor::new(&hierarchy);
    assert_eq!(iterator.map(|value| value.path().to_string()).collect::<Vec<_>>(), vec!["a::b::c::d::3", "a::b::c::d::2", "a::b::c::d::1", "a::b::c::d", "a::b::c", "a::b", "a"]);
}