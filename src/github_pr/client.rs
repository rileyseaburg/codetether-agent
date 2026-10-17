//! Application adapters for CLI arguments and A2A credential acquisition.
use super::body::merge_signature;
use super::context::{RepoContext, load_repo_context};
use crate::a2a::git_credentials::{GitCredentialQuery, request_git_credentials};
use crate::cli::CreatePrArgs;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::Serialize;
use std::path::PathBuf;
mod credentials;
mod request;
mod result;
use credentials::github_client;
pub use request::create_or_update;
pub use result::PullRequestResult;
