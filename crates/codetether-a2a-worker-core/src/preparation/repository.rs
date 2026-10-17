//! Repository setup before a worker task session executes.
use crate::task_config::TaskContext;
use anyhow::Result;
use codetether_provenance::{ensure_provenance_trailers, install_commit_msg_hook};
use codetether_worktree::repository::credentials::configure_repo_git_auth;
/// Prepare repository credentials, attribution hooks and requested branch.
///
/// # Arguments
/// * `task_id` - Task identifier for warning diagnostics.
/// * `directory` - Optional checkout directory.
/// * `provenance` - Optional execution attribution.
/// * `context` - Workspace ID and branch metadata.
/// # Returns
/// Unit after best-effort configuration and required branch preparation.
/// # Errors
/// Returns branch inspection or checkout errors; configuration failures are logged.
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
        tracing::warn!(target: "codetether_agent::a2a::worker::task_session::task_session_git", task_id, error = %error, "Failed to configure Git credential helper");
    }
    if let Some(directory) = directory {
        if let Err(error) = install_commit_msg_hook(directory) {
            tracing::warn!(target: "codetether_agent::a2a::worker::task_session::task_session_git", task_id, error = %error, "Failed to install commit-msg hook");
        }
        if let Some(provenance) = provenance
            && let Err(error) = ensure_provenance_trailers(directory, provenance)
        {
            tracing::warn!(target: "codetether_agent::a2a::worker::task_session::task_session_git", task_id, error = %error, "Failed to pre-seed provenance trailers");
        }
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
