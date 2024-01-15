// TODO: Organize traits.

pub mod has_parent;
pub mod has_root;
pub mod has_path_segment;
pub mod has_path_get;
pub mod has_visitor;
pub mod has_get;
pub mod has_relative_access;
pub mod has_value;

pub mod is_path_segment;
pub mod is_tree;
pub mod tree_update;
pub mod has_branches;
pub mod is_visitor;
pub mod has_root_visitor;

pub use has_value::*;
pub use is_visitor::*;
pub use has_branches::*;
pub use tree_update::*;
pub use has_parent::*;
pub use has_root::*;
pub use has_path_segment::*;
pub use has_get::*;
// pub use has_path_get::*;
pub use has_visitor::*;
pub use has_relative_access::*;
pub use has_root_visitor::*;

pub use is_path_segment::*;
pub use self::is_tree::*;
