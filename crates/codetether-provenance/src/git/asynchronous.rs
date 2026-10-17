//! Asynchronous provenance-aware Git commit.

use super::{
    commit_message_with_provenance,
    context::{apply_git_identity_env_tokio, enrich_repo_provenance},
};
use crate::{ExecutionProvenance, commit_args_with_signature_policy};
use anyhow::{Context, Result};
use std::path::Path;

/// Create a commit from staged changes using configured signing and provenance identity.
///
/// # Arguments
/// The repository path must contain staged changes; the message and optional provenance
/// determine commit text and author/committer environment.
///
/// # Returns
/// The Git process output, whose exit status reports whether the commit succeeded.
///
/// # Errors
/// Returns an error if the Git process cannot be spawned or waited on.
///
/// # Examples
/// ```no_run
/// use codetether_provenance::{git_commit_with_provenance};
/// # async fn example() -> anyhow::Result<()> {
/// let output = git_commit_with_provenance(std::path::Path::new("/tmp/checkout"),"fix: update",None).await?;
/// assert!(output.status.success());
/// # Ok(()) }
/// ```
pub async fn git_commit_with_provenance(
    repo_path: &Path,
    commit_msg: &str,
    provenance: Option<&ExecutionProvenance>,
) -> Result<std::process::Output> {
    let enriched = enrich_repo_provenance(provenance, repo_path);
    let message = commit_message_with_provenance(commit_msg, enriched.as_ref());
    let mut command = tokio::process::Command::new("git");
    command
        .args(commit_args_with_signature_policy())
        .arg(&message)
        .current_dir(repo_path);
    apply_git_identity_env_tokio(&mut command, enriched.as_ref());
    command
        .output()
        .await
        .context("Failed to execute git commit")
}
