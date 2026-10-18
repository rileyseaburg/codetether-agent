//! Point-in-time workspace file listings and Git metadata.
//!
//! [WorkspaceSnapshot::capture] combines shallow, pruned [WorkspaceEntry]
//! values with [detect_git_status]. [sort_entries] orders entries for display
//! without any dependency on a terminal renderer.
mod capture;
mod entries;
mod policy;
mod sort;
mod status;
mod types;
pub use entries::collect_entries;
pub use policy::should_skip_entry;
pub use sort::sort_entries;
pub use status::detect_git_status;
pub use types::{WorkspaceEntry, WorkspaceEntryKind, WorkspaceSnapshot};
