//! Repository-local credential-helper installation and Git configuration.
//!
//! Git commands, lock recovery, helper script generation and config updates
//! have separate operations. Credential broker requests remain with the host.
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, SystemTime};
mod configure_repo_git_auth;
pub use configure_repo_git_auth::configure_repo_git_auth;
mod run_git_config_command;
use run_git_config_command::run_git_config_command;
mod run_git_config_command_with_lock_recovery;
pub use run_git_config_command_with_lock_recovery::run_git_config_command_with_lock_recovery;
mod is_git_config_lock_error;
use is_git_config_lock_error::is_git_config_lock_error;
mod remove_stale_git_config_lock;
use remove_stale_git_config_lock::remove_stale_git_config_lock;
mod repo_git_path;
use repo_git_path::repo_git_path;
mod exclude_legacy_worktree_helper;
use exclude_legacy_worktree_helper::exclude_legacy_worktree_helper;
mod configure_repo_git_github_app;
pub use configure_repo_git_github_app::configure_repo_git_github_app;
mod run_git_command;
use run_git_command::run_git_command;
mod set_local_config;
use set_local_config::set_local_config;
mod write_git_credential_helper_script;
pub use write_git_credential_helper_script::write_git_credential_helper_script;
mod mark_executable;
#[cfg(unix)]
use mark_executable::mark_executable;
mod shell_single_quote;
use shell_single_quote::shell_single_quote;
