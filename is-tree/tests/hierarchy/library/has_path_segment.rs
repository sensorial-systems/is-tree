use is_tree::HasPathSegment;

use super::Library;

impl HasPathSegment for Library {
    fn path_segment(&self) -> &String {
        &self.name
    }
}
