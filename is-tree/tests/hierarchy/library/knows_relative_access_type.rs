use is_tree::KnowsRelativeAccessType;

use crate::hierarchy::{visitors::Visitors, Module};
use super::Library;

impl<'a> KnowsRelativeAccessType<'a> for Library {
    type RelativeType = Visitors<'a, &'a Library, &'a Module>;
}
