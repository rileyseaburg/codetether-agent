//! Workspace-scope helpers for worker registration and heartbeats.

use reqwest::Client;

use super::{WorkerTaskRuntime, task_timeline, workspace_resolve};

pub(super) async fn sync_timeline_to_runtime(
    timeline: &task_timeline::TaskTimeline,
    runtime: &WorkerTaskRuntime,
) {
    timeline.sync_progress().await;
    *runtime.task_progress.lock().await = timeline.progress_handle().lock().await.clone();
}

pub(super) async fn resolve_and_log_workspace_ids(
    client: &Client,
    server: &str,
    token: &Option<String>,
    workspace_roots: &[String],
) -> Vec<String> {
    match workspace_resolve::resolve_workspace_ids(client, server, token, workspace_roots).await {
        Ok(resolved) => resolved_workspace_ids(&resolved, workspace_roots),
        Err(error) => {
            tracing::warn!(error = %error, "Failed to resolve workspace IDs from server");
            Vec::new()
        }
    }
}

fn resolved_workspace_ids(
    resolved: &[workspace_resolve::ResolvedWorkspace],
    workspace_roots: &[String],
) -> Vec<String> {
    if resolved.is_empty() {
        tracing::warn!(roots = ?workspace_roots, "No server-side workspace IDs resolved for configured roots — the server may not route tasks to this worker. Ensure workspaces are registered with the control plane and that their paths fall under the configured roots.");
    } else {
        let ids: Vec<String> = resolved.iter().map(|ws| ws.id.clone()).collect();
        tracing::info!(workspace_ids = ?ids, count = resolved.len(), "Resolved server-side workspace IDs for configured roots");
    }
    resolved.iter().map(|ws| ws.id.clone()).collect()
}
