//! Application adapters for CLI arguments and A2A credential acquisition.
use anyhow::{Result, anyhow};
use codetether_a2a_http::git_credentials::{GitCredentialQuery, request_git_credentials};
use codetether_cli_options::CreatePrArgs;
use codetether_github_pr::body::merge_signature;
use codetether_github_pr::context::{RepoContext, load_repo_context};
use reqwest::Client;
use serde::Serialize;
use std::path::PathBuf;
mod credentials;
mod request;
mod result;
use credentials::github_client;
pub use request::create_or_update;
pub use result::PullRequestResult;
