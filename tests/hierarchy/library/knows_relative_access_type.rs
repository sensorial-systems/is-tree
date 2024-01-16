use is_tree::KnowsRelativeAccessType;

use crate::hierarchy::visitors::Visitors;
use super::Library;

impl<'a> KnowsRelativeAccessType for &'a Library {
    type RelativeType = Visitors<'a>;
}
