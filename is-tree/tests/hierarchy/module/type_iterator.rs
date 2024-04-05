use ::is_tree::*;
use super::super::*;

impl<'a> KnowsVisitorOf<'a, String> for &'a Module {
    type Visitor = Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>;
}

impl<'a> KnowsVisitorOf<'a, String> for &'a mut Module {
    type Visitor = Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>;
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> for &'a Module {
    fn type_iterator(self, parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> TypeIterator<Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> {
        let parent = parent.unwrap();
        let mut collection = Vec::new();
        collection.push(Visitor::new(parent.clone(), &self.name));
        collection.extend(self.children.iter().flat_map(|child| child.iter_type_with_parent::<String>(Some(Visitor::new(parent.clone().into(), self).into()))));
        collection.into()
    }
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> for &'a mut Module {
    fn type_iterator(self, parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> TypeIterator<Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> {
        let mut visitors = Vec::new();
        let parent = parent.unwrap();
        let self_as_parent: Visitor<Visitors<'a, &'a Library, &'a Module>, &'a Module> = Visitor::new(parent.clone().into(), unsafe { &mut *(self as *mut Module) }).into();
        visitors.push(Visitor::new(parent.clone(), &mut self.name));
        visitors.extend(self.children.iter_mut().flat_map(|child| child.iter_type_with_parent::<String>(Some(self_as_parent.clone().into()))));
        visitors.into()
    }
}

