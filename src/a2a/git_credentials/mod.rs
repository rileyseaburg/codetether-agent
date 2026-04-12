//! Git credential helpers for worker-managed repositories.
//!
//! This module handles credential broker requests, local helper script setup,
//! and Git/GitHub CLI wiring for repos managed through the A2A control plane.
//!
//! # Examples
//!
//! ```ignore
//! let query = GitCredentialQuery::default();
//! let creds = request_git_credentials(client, server, &token, None, "ws-1", "get", &query).await?;
//! ```

mod gh_cli;
mod gh_query;
mod git_config;
mod helper;
mod material;
mod query;
mod repo_config;
mod request;
mod script;
mod stdin_query;
#[cfg(test)]
mod tests;

pub use helper::run_git_credential_helper;
pub use material::GitCredentialMaterial;
pub use query::GitCredentialQuery;
pub use repo_config::{configure_repo_git_auth, configure_repo_git_github_app};
pub use request::request_git_credentials;
pub use script::write_git_credential_helper_script;
