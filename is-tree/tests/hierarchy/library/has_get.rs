use is_tree::HasGet;

use super::Library;

// TODO: derive HasGet
impl<'a> HasGet<'a> for &'a Library {}