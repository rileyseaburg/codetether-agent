//! Control-plane workspace records and local clone-path resolution.
//!
//! [fetch_workspace_record] reads metadata; [resolve_task_workspace_dir]
//! resolves an optional task workspace. [resolve_workspace_clone_path] applies
//! the clone-root policy shared by worker cloning and session setup.
mod clone_path;
mod fetch;
mod record;
mod resolve;
pub use clone_path::{git_clone_base_dir, resolve_workspace_clone_path};
pub use fetch::fetch_workspace_record;
pub use record::RegisteredWorkspaceRecord;
pub use resolve::resolve_task_workspace_dir;

mod catalog;
pub use catalog::{ResolvedWorkspace, resolve_workspace_ids};
mod register;
pub use register::register_cloned_workspace;

mod sync;
pub use sync::sync_workspaces_from_server;

mod scope;
pub use scope::resolve_and_log_workspace_ids;
mod sync_start;
pub use sync_start::start_workspace_sync;
