use ::is_tree::*;

use crate::hierarchy::{Library, Visitors};

pub type ModuleVisitor<'a, Module> = Visitor<Visitors<'a, &'a Library, Module>, Module>;
