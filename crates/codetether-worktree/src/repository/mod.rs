//! Repository preparation and publication preconditions.
//!
//! Clone targets, push refspecs, and declared hooks are validated separately.
//! The host supplies the active hooks directory through its Git adapter.
mod clone_target;
mod git_refspec;
mod hooks;
pub use clone_target::prepare_clone_target;
pub use git_refspec::push_refspec;
pub use hooks::require_installed;

/// Repository-local credential helper scripts and Git configuration.
pub mod credentials;

/// Git command execution and repository preparation.
pub mod git;

/// Hook-checked commit and push orchestration with execution attribution.
pub mod publication;
