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

mod helper;
use codetether_a2a_http::git_credentials as material;
use codetether_a2a_http::git_credentials as query;
use codetether_a2a_http::git_credentials as request;
use codetether_worktree::repository::credentials as script;

pub use codetether_worktree::repository::credentials::{
    configure_repo_git_auth, configure_repo_git_github_app,
};
pub use helper::run_git_credential_helper;
pub use material::GitCredentialMaterial;
pub use query::GitCredentialQuery;
pub use request::request_git_credentials;
pub use script::write_git_credential_helper_script;
