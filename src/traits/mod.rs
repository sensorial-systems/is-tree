// TODO: Organize traits.

pub mod knows_parent;
pub mod knows_parent_visitor;
pub mod has_parent;
pub mod has_root;
pub mod has_path_segment;
pub mod has_path_get;
pub mod has_visitor;
pub mod has_get;

pub mod is_path_segment;
pub mod is_tree;

pub use knows_parent::*;
pub use knows_parent_visitor::*;
pub use has_parent::*;
pub use has_root::*;
pub use has_path_segment::*;
pub use has_path_get::*;
pub use has_visitor::*;

pub use is_path_segment::*;
pub use is_tree::*;
