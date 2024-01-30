use is_tree::KnowsRelativeAccessType;

use crate::hierarchy::{visitors::Visitors, Library};

use super::Module;

impl<'a> KnowsRelativeAccessType<'a> for Module {
    type RelativeType = Visitors<'a, &'a Library, &'a Module>;
}
