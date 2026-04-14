//! Workspace resolution helpers for the A2A worker.
//!
//! These helpers convert a claimed workspace identifier into the local clone
//! directory used by worker tools.

mod clone_path;
mod resolve;

pub use clone_path::{git_clone_base_dir, resolve_workspace_clone_path};
pub use resolve::resolve_task_workspace_dir;
