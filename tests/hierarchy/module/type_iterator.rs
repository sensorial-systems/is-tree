use ::is_tree::*;
use super::super::*;

impl<'a> KnowsVisitorFor<'a, &'a Module> for &'a String {
    type Visitor = Visitor<Visitors<'a>, &'a String>;
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a>, &'a String>> for &'a Module {
    fn type_iterator(self, parent: Option<Visitors<'a>>) -> TypeIterator<Visitor<Visitors<'a>, &'a String>> {
        let mut collection = Vec::new();
        let parent = parent.unwrap();
        let visitor = Visitor::new(parent.clone(), &self.name);
        collection.push(visitor.clone());
        collection.extend(self.children.iter().flat_map(|child| child.iter_type_with_parent::<&String>(Some(parent.clone()))));
        collection.into()
    }
}
