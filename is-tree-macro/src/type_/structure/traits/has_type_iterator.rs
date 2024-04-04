use quote::quote;

use crate::{traits::AttributeQuery, type_::Structure};



pub fn impl_has_type_iterator(structure: &Structure) -> proc_macro2::TokenStream {
    if structure.has_attribute(vec!["tree", "type_iterator"]) {
        let name = &structure.name;
        quote! {
            impl<'a> ::is_tree::TypeIter<'a, ::is_tree::Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> for &'a #name {
                fn type_iterator(self, _parent: Option<Visitors<'a, &'a Library, &'a Module>>) -> ::is_tree::TypeIterator<::is_tree::Visitor<Visitors<'a, &'a Library, &'a Module>, &'a String>> {
                    use ::is_tree::{IterType, HasVisitor};
                    let mut collection = Vec::new();
                    let visitor = ::is_tree::Visitor::new(self.visitor().into(), &self.name);
                    collection.push(visitor.clone());
                    collection.extend((&self.root_module).iter_type_with_parent::<String>(Some(self.visitor().into())));
                    collection.into()
                }
            }
        }
    } else {
        quote! {}
    }
}