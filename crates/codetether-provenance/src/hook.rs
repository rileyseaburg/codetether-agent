//! Commit-message hook installation, generation, and trailer updates.
//!
//! [install_commit_msg_hook] installs the generated script; [ensure_provenance_trailers]
//! updates the current commit message. Path helpers resolve Git's chosen locations.
mod install;
mod paths;
mod permissions;
mod script;
mod seed;
pub use install::install_commit_msg_hook;
pub use paths::{commit_editmsg_path, repo_root};
pub use script::commit_msg_hook_script;
pub use seed::ensure_provenance_trailers;
