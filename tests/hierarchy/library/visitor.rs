use is_tree::RootVisitor;
use super::Library;

pub type LibraryVisitor<'a> = RootVisitor<&'a Library>;
