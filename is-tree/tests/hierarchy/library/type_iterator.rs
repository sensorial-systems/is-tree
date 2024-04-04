use ::is_tree::*;
use super::*;

impl<'a> TypeIter<'a, Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> for &'a mut Library {
    fn type_iterator(self, _parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> TypeIterator<Visitor<Visitors<'a, &'a Library, &'a Module>, &'a mut String>> {
        let mut collection = Vec::new();
        // FIXME: This is a workaround. We can wrap this in a safe function.
        let value = unsafe { &mut *(&mut self.name as *mut String) };
        let self_ = unsafe { &mut *(self as *mut Library) };
        let parent: Visitors<'_, &Library, &Module> = self.visitor().into();
        let visitor = Visitor::new(parent.clone(), value);
        collection.push(visitor);
        collection.extend((&mut self_.root_module).iter_type_with_parent::<String>(Some(parent)));
        collection.into()
    }
}
