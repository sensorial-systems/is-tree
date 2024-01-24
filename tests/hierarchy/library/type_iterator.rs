use ::is_tree::*;
use super::*;

impl<'a> KnowsVisitorFor<'a, Library> for String {
    type Visitor = Visitor<Visitors<'a>, &'a String>;
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a>, &'a String>> for Library {
    fn type_iterator(&'a self, _parent: Option<Visitors<'a>>) -> TypeIterator<Visitor<Visitors<'a>, &'a String>> {
        let mut collection = Vec::new();
        let visitor = Visitor::new(self.visitor().into(), &self.name);
        collection.push(visitor.clone());
        collection.extend(self.root_module.iter_type_with_parent::<String>(Some(self.visitor().into())));
        collection.into()
    }
}
