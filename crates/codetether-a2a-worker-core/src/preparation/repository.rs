//! Repository setup before a worker task session executes.
use crate::task_config::TaskContext;
use anyhow::{Context, Result};
use codetether_provenance::ensure_provenance_trailers;
use codetether_worktree::repository::credentials::configure_repo_git_auth;
use codetether_worktree::repository::{install_declared_hooks, publication::require_installed};

const TARGET: &str = "codetether_agent::a2a::worker::task_session::task_session_git";

/// Prepare repository credentials, declared and attribution hooks, and branch.
///
/// The repository's own hooks are installed first (its pre-commit config and
/// pre-push cache gate), then CodeTether's provenance `commit-msg` hook, which
/// never replaces a repository-owned `commit-msg` hook. The declared-hook
/// policy is then verified, so a task cannot commit or push without them.
///
/// # Arguments
/// * `task_id` - Task identifier for diagnostics.
/// * `directory` - Optional checkout directory.
/// * `provenance` - Optional execution attribution.
/// * `context` - Workspace ID and branch metadata.
/// # Returns
/// Unit once the checkout is ready for the task.
/// # Errors
/// Refuses the task when declared hooks cannot be installed or verified, and
/// returns branch inspection or checkout errors. Credential, provenance-hook
/// and trailer configuration failures are logged.
/// # Examples
/// ```rust,no_run
/// # async fn example(context: &codetether_a2a_worker_core::task_config::TaskContext) -> anyhow::Result<()> {
/// use codetether_a2a_worker_core::preparation::prepare_git;
/// prepare_git("task", None, None, context).await?;
/// # Ok(()) }
/// ```
pub async fn prepare_git(
    task_id: &str,
    directory: Option<&std::path::Path>,
    provenance: Option<&codetether_provenance::ExecutionProvenance>,
    context: &TaskContext,
) -> Result<()> {
    if let (Some(directory), Some(workspace_id)) = (directory, context.workspace_id.as_deref())
        && directory.join(".git").exists()
        && let Err(error) = configure_repo_git_auth(directory, workspace_id)
    {
        tracing::warn!(target: TARGET, task_id, error = %error, "Failed to configure Git credential helper");
    }
    if let Some(directory) = directory
        && directory.join(".git").exists()
    {
        prepare_hooks(task_id, directory, provenance).await?;
    }
    codetether_worktree::repository::git::ensure_metadata_checked_out(
        directory,
        crate::task_config::metadata_str(
            &context.metadata,
            &["branch_name", "git_branch", "pr_head"],
        ),
    )
    .await
}

async fn prepare_hooks(
    task_id: &str,
    directory: &std::path::Path,
    provenance: Option<&codetether_provenance::ExecutionProvenance>,
) -> Result<()> {
    let installed = install_declared_hooks(directory).await.inspect_err(|error| {
        tracing::error!(target: TARGET, task_id, error = %format!("{error:#}"), "Refusing task: repository hooks could not be installed");
    })?;
    if !installed.is_empty() {
        tracing::info!(target: TARGET, task_id, installers = ?installed, "Installed repository-declared hooks");
    }
    match super::provenance_hook::install_provenance_hook(directory).await {
        Ok(true) => {}
        Ok(false) => {
            tracing::info!(target: TARGET, task_id, "Kept repository-owned commit-msg hook; provenance trailers are pre-seeded")
        }
        Err(error) => {
            tracing::warn!(target: TARGET, task_id, error = %error, "Failed to install commit-msg hook")
        }
    }
    if let Some(provenance) = provenance
        && let Err(error) = ensure_provenance_trailers(directory, provenance)
    {
        tracing::warn!(target: TARGET, task_id, error = %error, "Failed to pre-seed provenance trailers");
    }
    require_installed(directory)
        .await
        .context("repository hooks could not be installed")
}
