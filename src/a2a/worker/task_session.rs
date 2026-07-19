//! Session preparation for claimed tasks.

use anyhow::Result;

use crate::{provenance::ClaimProvenance, session::Session};

use super::task_session_resolve::{normalize_agent, resolve_workspace};
use super::{TaskContext, WorkerTaskRuntime, task_timeline};

mod task_session_git;
mod task_session_load;

pub(super) async fn prepare_task_session(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    context: &TaskContext,
    timeline: &mut task_timeline::TaskTimeline,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
) -> Result<(Session, String)> {
    let mut session = task_session_load::load(context).await?;
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
