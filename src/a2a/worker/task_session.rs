//! Native entry for shared claimed-task session preparation.
mod host;
mod task_session_load;
use super::{TaskContext, WorkerTaskRuntime, task_timeline};
use crate::{provenance::ClaimProvenance, session::Session};
pub(super) async fn prepare_task_session(
    runtime: &WorkerTaskRuntime,
    task_id: &str,
    context: &TaskContext,
    timeline: &mut task_timeline::TaskTimeline,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
) -> anyhow::Result<(Session, String)> {
    codetether_a2a_worker_core::preparation::session::prepare(
        &host::Native(runtime),
        task_id,
        context,
        timeline,
        claim_provenance,
        provider_keys,
    )
    .await
}
