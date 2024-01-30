use is_tree::HasGet;

use super::Module;

// TODO: derive HasGet
impl<'a> HasGet<'a> for &'a Module {}
impl<'a> HasGet<'a> for &'a mut Module {}
