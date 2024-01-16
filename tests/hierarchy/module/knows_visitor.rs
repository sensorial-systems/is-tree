use is_tree::KnowsVisitor;

use super::{ModuleVisitor, Module};

impl<'a> KnowsVisitor for &'a Module {
    type Visitor = ModuleVisitor<'a>;
}
