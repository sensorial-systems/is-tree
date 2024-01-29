use ::is_tree::*;
use super::super::*;

impl<'a> KnowsVisitorFor<'a, &'a Module> for &'a String {
    type Visitor = Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>;
}

impl<'a> KnowsVisitorFor<'a, &'a mut Module> for &'a mut String {
    type Visitor = Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>;
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> for &'a Module {
    fn type_iterator(self, parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> TypeIterator<Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> {
        let mut collection = Vec::new();
        let parent = parent.unwrap();
        let visitor: Visitor<Visitors<'_, &Library, &Module>, &String> = Visitor::new(parent.clone(), &self.name);
        collection.push(visitor.clone());
        // FIXME: The parent seems incorrect.
        // Correct one should be:
        // let parent: Visitors<'_, &Library, &Module> = visitor.into();
        collection.extend(self.children.iter().flat_map(|child| child.iter_type_with_parent::<&String>(Some(parent.clone()))));
        collection.into()
    }
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> for &'a mut Module {
    fn type_iterator(self, parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> TypeIterator<Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> {
        let mut collection = Vec::new();
        let parent = parent.unwrap();
        let visitor = Visitor::new(parent.clone(), &mut self.name);
        collection.push(visitor);
        // FIXME: The parent seems incorrect.
        collection.extend(self.children.iter_mut().flat_map(|child| child.iter_type_with_parent::<&mut String>(Some(parent.clone()))));
        collection.into()
    }
}

