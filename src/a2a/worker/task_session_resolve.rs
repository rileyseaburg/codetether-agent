//! Adapt application session workspace metadata to shared preparation.
use super::{TaskContext, WorkerTaskRuntime, task_timeline};
use crate::session::Session;
use anyhow::Result;
pub(super) use codetether_a2a_worker_core::preparation::normalize_agent;
pub(super) async fn resolve_workspace(
    runtime: &WorkerTaskRuntime,
    context: &TaskContext,
    session: &mut Session,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<()> {
    codetether_a2a_http::workspace::resolve_session_workspace(
        &runtime.client,
        &runtime.server,
        &runtime.token,
        context,
        &mut session.metadata.directory,
        timeline,
    )
    .await
}
