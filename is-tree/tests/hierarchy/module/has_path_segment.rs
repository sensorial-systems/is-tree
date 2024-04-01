use is_tree::HasPathSegment;

use super::Module;

impl HasPathSegment for Module {
    fn path_segment(&self) -> &String {
        &self.name
    }
}
