//! Repository inspection and execution-signature context for PR creation.
//! [load_repo_context] reads Git state and returns [RepoContext] with [SignatureContext].
use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use std::process::Command;
mod repo;
mod signature;
pub use repo::RepoContext;
pub use signature::SignatureContext;
mod load_repo_context;
pub use load_repo_context::load_repo_context;
mod env_or_trailer;
use env_or_trailer::env_or_trailer;
mod git_stdout;
use git_stdout::git_stdout;
mod git_config;
use git_config::git_config;
mod commit_trailer;
use commit_trailer::commit_trailer;
mod parse_github_remote;
use parse_github_remote::parse_github_remote;
#[cfg(test)]
mod tests;
