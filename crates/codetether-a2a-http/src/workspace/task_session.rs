//! Resolve the checkout directory used by a worker task session.
use super::resolve_task_workspace_dir;
use anyhow::Result;
use codetether_a2a_worker_core::{task_config::TaskContext, task_timeline};
/// Apply task workspace policy and report workspace readiness.
///
/// # Arguments
/// * `client` - HTTP client for workspace resolution.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `context` - Task workspace selection policy.
/// * `directory` - Current session directory, updated in place.
/// * `timeline` - Timeline receiving a workspace-ready checkpoint.
/// # Returns
/// Unit after applying virtual-task policy or resolving a workspace.
/// # Errors
/// Returns control-plane workspace lookup errors.
/// # Examples
/// ```rust,no_run
/// # async fn example(context: &codetether_a2a_worker_core::task_config::TaskContext) -> anyhow::Result<()> {
/// use codetether_a2a_http::workspace::resolve_session_workspace;
/// let mut directory = None;
/// let mut timeline = codetether_a2a_worker_core::task_timeline::TaskTimeline::new("task", 60);
/// resolve_session_workspace(&reqwest::Client::new(), "http://localhost:8001", &None, context, &mut directory, &mut timeline).await?;
/// # Ok(()) }
/// ```

pub async fn resolve_session_workspace(
    client: &reqwest::Client,
    server: &str,
    token: &Option<String>,
    context: &TaskContext,
    directory: &mut Option<std::path::PathBuf>,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    if context.is_virtual_task {
        if !context.preserve_session_workspace {
            *directory = None;
        }
        return Ok(());
    }
    if let Some(workspace_path) =
        resolve_task_workspace_dir(client, server, token, context.workspace_id.as_deref()).await?
    {
        *directory = Some(workspace_path);
    }
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::WorkspaceReady,
        directory.as_deref().map(|p| p.display().to_string()),
    );
    Ok(())
}
