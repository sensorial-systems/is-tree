use ::is_tree::*;
use super::super::*;

impl<'a> KnowsVisitorFor<'a, &'a Module> for String {
    type Visitor = Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>;
}

impl<'a> KnowsVisitorFor<'a, &'a mut Module> for String {
    type Visitor = Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>;
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> for &'a Module {
    fn type_iterator(self, parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> TypeIterator<Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> {
        let mut collection = Vec::new();
        let parent = parent.unwrap();
        let visitor: Visitor<Visitors<'_, &Library, &Module>, &String> = Visitor::new(parent.clone(), &self.name);
        collection.push(visitor.clone());
        let parent: Visitors<'_, &Library, &Module> = Visitor::new(parent.clone().into(), self).into();
        collection.extend(self.children.iter().flat_map(|child| child.iter_type_with_parent::<String>(Some(parent.clone()))));
        collection.into()
    }
}

impl<'a> TypeIter<'a, Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> for &'a mut Module {
    fn type_iterator(self, parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> TypeIterator<Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> {
        let mut collection = Vec::new();
        let parent = parent.unwrap();
        // FIXME: This is a workaround. We can wrap this in a safe function.
        let self_ = unsafe { &mut *(self as *mut Module) };
        let visitor = Visitor::new(parent.clone(), &mut self.name);
        collection.push(visitor);
        let parent: Visitor<ModuleParentVisitor<'_>, &Module> = Visitor::new(parent.clone().into(), self_).into();
        let parent: Visitors<'_, &Library, &Module> = parent.into();
        collection.extend(self.children.iter_mut().flat_map(|child| child.iter_type_with_parent::<String>(Some(parent.clone()))));
        collection.into()
    }
}

