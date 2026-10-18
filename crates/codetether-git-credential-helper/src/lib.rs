//! Git credential-helper execution and GitHub CLI delegation.
//!
//! [run_git_credential_helper] consumes Git's stdin query, requests short-lived
//! broker credentials and emits the credential response. CLI options and broker
//! protocol types remain in their dedicated crates.
mod gh_cli;
mod gh_query;
mod helper;
mod stdin_query;
use codetether_a2a_http::git_credentials::{
    GitCredentialMaterial, GitCredentialQuery, request_git_credentials,
};
pub use helper::run_git_credential_helper;
#[cfg(test)]
mod tests;
