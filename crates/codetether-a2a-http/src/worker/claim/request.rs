//! Claim a task and update its shared progress timeline.
use super::*;
/// Submit a task claim, returning its attribution when accepted.
///
/// # Arguments
/// * `client` - HTTP client.
/// * `server` - Control-plane URL.
/// * `token` - Optional bearer token.
/// * `worker_id` - Claiming worker identifier.
/// * `agent_name` - Principal used for proof signing.
/// * `task_id` - Task being claimed.
/// * `timeline` - Progress timeline updated at request, rejection and acceptance.
/// # Returns
/// Claim data on success, or `None` for a rejected HTTP response.
/// # Errors
/// Returns signing, transport, or successful-response decoding errors.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_a2a_http::worker::claim::claim_task;
/// let mut timeline = codetether_a2a_worker_core::task_timeline::TaskTimeline::new("task", 60);
/// let claimed = claim_task(&reqwest::Client::new(), "http://localhost:8001", &None, "worker", "agent", "task", &mut timeline).await?;
/// # Ok(()) }
/// ```

pub async fn claim_task(
    client: &reqwest::Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    agent_name: &str,
    task_id: &str,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<Option<ClaimedTaskData>> {
    timeline.checkpoint(task_timeline::TaskCheckpoint::ClaimRequested);
    let request =
        super::super::build_claim_request(client, server, token, worker_id, agent_name, task_id)?;
    let response = request
        .json(&serde_json::json!({ "task_id": task_id }))
        .send()
        .await?;
    if !response.status().is_success() {
        return super::rejected::reject(task_id, response, timeline).await;
    }
    let claim = response.json::<TaskClaimResponse>().await?;
    if let Some(timeout) = claim.task_timeout_seconds.map(|v| v.clamp(60, 604800)) {
        timeline.update_timeout_secs(timeout);
    }
    let provider_keys = claim.provider_keys.clone();
    let claim_provenance = claim.into_provenance();
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::Claimed,
        Some(format!(
            "run_id={:?} attempt_id={:?}",
            claim_provenance.run_id, claim_provenance.attempt_id
        )),
    );
    tracing::info!(target: "codetether_agent::a2a::worker::task_claim", task_id, run_id = ?claim_provenance.run_id, attempt_id = ?claim_provenance.attempt_id, "Claimed task");
    Ok(Some(ClaimedTaskData {
        claim_provenance,
        provider_keys,
    }))
}
