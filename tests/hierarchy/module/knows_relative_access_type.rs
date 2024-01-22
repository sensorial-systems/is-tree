use is_tree::KnowsRelativeAccessType;

use crate::hierarchy::visitors::Visitors;

use super::Module;

impl<'a> KnowsRelativeAccessType<'a> for &'a Module {
    type RelativeType = Visitors<'a>;
}

impl<'a> KnowsRelativeAccessType<'a> for Module {
    type RelativeType = Visitors<'a>;
}
