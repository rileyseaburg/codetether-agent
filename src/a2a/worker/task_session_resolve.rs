//! Workspace resolution and agent normalization for task sessions.

use anyhow::Result;

use crate::session::Session;

use super::{TaskContext, WorkerTaskRuntime, resolve_task_workspace_dir, task_timeline};

pub(super) async fn resolve_workspace(
    runtime: &WorkerTaskRuntime,
    context: &TaskContext,
    session: &mut Session,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    if context.is_virtual_task {
        if !context.preserve_session_workspace {
            session.metadata.directory = None;
        }
        return Ok(());
    }
    if let Some(workspace_path) = resolve_task_workspace_dir(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        context.workspace_id.as_deref(),
    )
    .await?
    {
        session.metadata.directory = Some(workspace_path);
    }
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::WorkspaceReady,
        session
            .metadata
            .directory
            .as_deref()
            .map(|p| p.display().to_string()),
    );
    Ok(())
}

pub(super) fn normalize_agent(raw_agent: &str) -> String {
    if super::is_swarm_agent(raw_agent) || matches!(raw_agent, "build" | "plan") {
        return raw_agent.to_string();
    }
    tracing::info!(agent = %raw_agent, "Falling back to build agent");
    "build".to_string()
}
