//! Exclusive filesystem locks for per-workspace clone operations.
//!
//! [workspace_clone_lock_path] sanitizes the workspace ID. Retain the file
//! returned by [acquire_workspace_clone_lock] while cloning, then pass it to
//! [release_workspace_clone_lock]. Hosts supply configuration lazily.
mod acquire;
mod path;
mod release;
mod stale;
pub use acquire::acquire_workspace_clone_lock;
pub use path::workspace_clone_lock_path;
pub use release::release_workspace_clone_lock;
