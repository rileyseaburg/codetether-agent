//! # CodeTether paths
//!
//! Filesystem location rules shared by CodeTether crates:
//!
//! - [`data_dir`] — workspace-scoped data directory resolution.
//! - [`detect_workspace_root`] — nearest `.git` ancestor, never `/` or `$HOME`.
//! - [`is_unsafe_workspace_root`] — roots too broad to scope a workspace.
//!
//! # Examples
//!
//! ```rust
//! use std::path::Path;
//!
//! assert!(codetether_paths::is_unsafe_workspace_root(Path::new("/")));
//! ```

mod data_dir;
mod guard;

pub use data_dir::{
    DATA_DIR_ENV, data_dir, data_dir_for_workspace, detect_workspace_root, workspace_data_dir_from,
};
pub use guard::is_unsafe_workspace_root;

/// Platform project directories for CodeTether (`ai.codetether.codetether-agent`).
pub fn project_dirs() -> Option<directories::ProjectDirs> {
    directories::ProjectDirs::from("ai", "codetether", "codetether-agent")
}
