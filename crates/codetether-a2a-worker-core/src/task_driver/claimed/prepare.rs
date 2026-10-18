use super::Host;
use crate::{preparation::session::SessionState, task_timeline};
use codetether_provenance::ClaimProvenance;

use crate::task_config::TaskContext;
pub(super) async fn prepare<H: Host>(
    host: &H,
    task_id: &str,
    title: &str,
    context: &TaskContext,
    timeline: &mut task_timeline::TaskTimeline,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
) -> anyhow::Result<(H::Session, String)> {
    let (mut session, agent_type) = host
        .prepare(task_id, context, timeline, claim_provenance, provider_keys)
        .await?;
    host.preseed(&mut session, title);
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::AgentStarting,
        Some(format!(
            "agent={} model={:?}",
            agent_type,
            session.metadata().model
        )),
    );
    host.sync(timeline).await;
    Ok((session, agent_type))
}
