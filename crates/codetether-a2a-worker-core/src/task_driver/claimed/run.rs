use super::Host;
use crate::{outcome::TaskInner, task_timeline};
use codetether_provenance::ClaimProvenance;

/// Execute a claimed task through session preparation, agent work and finalization.
///
/// # Arguments
/// Receives native services, the claimed task and identity, its title and claim,
/// optional provider credentials, and the timeline updated as execution proceeds.
///
/// # Returns
/// The terminal task status, optional output/error and final session ID.
///
/// # Errors
/// Propagates special-task, session preparation and finalization errors. Agent
/// execution errors become failed task outcomes with their original error text.
///
/// # Examples
/// ```ignore
/// let result = execute(&host, &task, task_id, title, &claim, keys, &mut timeline).await?;
/// ```
pub async fn execute<H: Host>(
    host: &H,
    task: &serde_json::Value,
    task_id: &str,
    title: &str,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
    timeline: &mut task_timeline::TaskTimeline,
) -> TaskInner {
    let context = crate::task_config::build_task_context(task, title);
    timeline.checkpoint(task_timeline::TaskCheckpoint::MetadataParsed);
    if let Some(result) = host.special(task, title, &context).await {
        return result;
    }
    let (mut session, agent_type) = super::prepare::prepare(
        host,
        task_id,
        title,
        &context,
        timeline,
        claim_provenance,
        provider_keys,
    )
    .await?;
    let output = host.output(task_id);
    host.install(task_id);
    let (status, result, error, session_id) = host
        .execute(&mut session, &context, &agent_type, output)
        .await
        .unwrap_or_else(|err| ("failed", None, Some(format!("Error: {err}")), None));
    host.clear();
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::AgentDone,
        Some(format!("status={status}")),
    );
    host.sync(timeline).await;
    super::finish::finish(
        &mut session,
        task_id,
        status,
        result,
        error,
        session_id,
        context.is_virtual_task,
        timeline,
    )
    .await
}
