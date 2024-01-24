use super::*;

#[test]
fn tree_iterator() {
    let library = library();
    let iterator: TreeIterator<Visitors<'_>> = TreeIterator::new(&library);
    assert_eq!(iterator.map(|value| value.path().to_string()).collect::<Vec<_>>(), vec!["a::b::c::d::3", "a::b::c::d::2", "a::b::c::d::1", "a::b::c::d", "a::b::c", "a::b", "a"]);
}

#[test]
fn type_iterator() {
    let library = library();
    assert_eq!(library.iter_type::<String>().map(|visitor| visitor.value()).collect::<Vec<_>>(), vec!["b", "d", "1", "2", "3", "c", "a"]);
}