use is_tree::KnowsVisitor;

use super::{ModuleVisitor, Module};

impl<'a> KnowsVisitor<'a> for &'a Module {
    type Visitor = ModuleVisitor<'a, &'a Module>;
}

impl<'a> KnowsVisitor<'a> for Module {
    type Visitor = ModuleVisitor<'a, &'a Module>;
}
