use super::*;

#[test]
fn tree_iterator() {
    let library = library();
    let iterator: TreeIterator<Visitors<'_, &Library, &Module>> = TreeIterator::new(&library);
    assert_eq!(iterator.map(|value| value.path().to_string()).collect::<Vec<_>>(), vec!["a::b::c::d::3", "a::b::c::d::2", "a::b::c::d::1", "a::b::c::d", "a::b::c", "a::b", "a"]);
}

#[test]
fn type_iterator() {
    let mut library = library();

    // Test if parents are properly set.
    // Root's parent == self.
    assert_eq!((&library).iter_type::<String>().map(|visitor| visitor.parent().path().to_string()).collect::<Vec<_>>(), vec!["a", "a::b::c", "a::b::c::d", "a::b::c::d", "a::b::c::d", "a::b", "a"]);
    assert_eq!((&mut library).iter_type::<String>().map(|visitor| visitor.parent().path().to_string()).collect::<Vec<_>>(), vec!["a", "a::b::c", "a::b::c::d", "a::b::c::d", "a::b::c::d", "a::b", "a"]);

    assert_eq!((&library).iter_type::<String>().map(|visitor| *(&visitor).value()).collect::<Vec<_>>(), vec!["b", "d", "1", "2", "3", "c", "a"]);
    (&mut library).iter_type::<String>().for_each(|mut visitor| **(&mut visitor).value() = "x".to_string());
    assert_eq!((&library).iter_type::<String>().map(|visitor| *(&visitor).value()).collect::<Vec<_>>(), vec!["x", "x", "x", "x", "x", "x", "x"]);
}