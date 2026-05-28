//! Session preparation for claimed tasks.

use anyhow::Result;

use crate::{provenance::ClaimProvenance, session::Session};

use super::{TaskContext, WorkerTaskRuntime, resolve_task_workspace_dir, task_timeline};

mod task_session_git;

pub(super) async fn prepare_task_session(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    context: &TaskContext,
    timeline: &mut task_timeline::TaskTimeline,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
) -> Result<(Session, String)> {
    let mut session = match &context.resume_session_id {
        Some(session_id) => Session::load(session_id)
            .await
            .unwrap_or(Session::new().await?),
        None => Session::new().await?,
    };
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::SessionReady,
        Some(format!("session_id={}", session.id)),
    );
    resolve_workspace(runtime, context, &mut session, timeline).await?;
    let agent_type = normalize_agent(&context.raw_agent);
    session.set_agent_name(agent_type.clone());
    session.attach_claim_provenance(claim_provenance);
    session.metadata.provider_keys = provider_keys;
    if let Some(model) = context.selected_model.clone() {
        session.metadata.model = Some(model);
    }
    if !context.is_virtual_task {
        task_session_git::prepare_git(task_id, &mut session, context).await?;
        timeline.checkpoint(task_timeline::TaskCheckpoint::GitHookInstalled);
    }
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::ModelSelected,
        session.metadata.model.clone(),
    );
    Ok((session, agent_type))
}

async fn resolve_workspace(
    runtime: &WorkerTaskRuntime,
    context: &TaskContext,
    session: &mut Session,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    if context.is_virtual_task {
        session.metadata.directory = None;
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
            .map(|path| path.display().to_string()),
    );
    Ok(())
}

fn normalize_agent(raw_agent: &str) -> String {
    if super::is_swarm_agent(raw_agent) || matches!(raw_agent, "build" | "plan") {
        return raw_agent.to_string();
    }
    tracing::info!(agent = %raw_agent, "Falling back to build agent");
    "build".to_string()
}
