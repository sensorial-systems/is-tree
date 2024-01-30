use is_tree::HasGet;

use super::Module;

impl<'a> HasGet<'a> for &'a Module {}
