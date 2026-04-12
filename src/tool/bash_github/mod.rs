//! GitHub auth helpers for bash tool execution.
//!
//! This module prepares repo-scoped GitHub CLI environment variables for bash
//! commands that talk to GitHub. It keeps credential lookup and `gh`
//! configuration discovery separate from command execution.
//!
//! # Examples
//!
//! ```ignore
//! let auth = load_github_command_auth("gh pr create", Some(".")).await?;
//! assert!(auth.is_some());
//! ```

mod auth;
mod command;
mod config;
mod credentials;
mod loader;
mod remote;
mod repo_context;

pub use auth::GitHubCommandAuth;
pub use loader::load_github_command_auth;
