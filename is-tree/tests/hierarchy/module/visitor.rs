use ::is_tree::*;

use crate::hierarchy::Visitors;

// TODO: Can we get rid of this?
pub type ModuleVisitor<Library, Module> = Visitor<Visitors<Library, Module>, Module>;
