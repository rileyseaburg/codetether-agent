//! Assemble full and fallback worker heartbeat payloads.
use super::*;
/// Build one heartbeat tick after updating the shared worker status.
///
/// # Arguments
/// * `client` - HTTP client for optional cognition retrieval.
/// * `heartbeat_state` - Shared worker status to update and snapshot.
/// * `cognition_config` - Settings for optional cognition inclusion.
/// * `task_progress` - Shared active-task progress.
/// * `active_count` - Current number of processing tasks.
/// * `cognition_disabled_until` - Optional retry backoff deadline.
/// # Returns
/// Full heartbeat, base fallback, active progress and cognition inclusion flag.
/// # Examples
/// ```rust,no_run
/// # async fn example(state: &codetether_a2a_worker_core::HeartbeatState, config: &codetether_a2a_worker_core::CognitionHeartbeatConfig, progress: &std::sync::Arc<tokio::sync::Mutex<codetether_a2a_worker_core::task_timeline::TaskProgressState>>) {
/// use codetether_a2a_http::worker::heartbeat_snapshot::build_heartbeat_payload;
/// let tick = build_heartbeat_payload(&reqwest::Client::new(), state, config, progress, 0, None).await;
/// assert!(tick.base_payload.get("worker_id").is_some());
/// # }
/// ```

pub async fn build_heartbeat_payload(
    client: &Client,
    heartbeat_state: &HeartbeatState,
    cognition_config: &CognitionHeartbeatConfig,
    task_progress: &Arc<Mutex<TaskProgressState>>,
    active_count: usize,
    cognition_disabled_until: Option<Instant>,
) -> HeartbeatPayload {
    let (status, sub_agents) = state::snapshot(heartbeat_state, active_count).await;
    let progress = task_progress.lock().await.clone();
    let task_progress_payload = progress::payload(progress);
    let base_payload = serde_json::json!({"worker_id": &heartbeat_state.worker_id, "agent_name": &heartbeat_state.agent_name, "status": status, "active_task_count": active_count, "sub_agents": sub_agents});
    let mut payload = base_payload.clone();
    let cognition_allowed = cognition_disabled_until
        .map(|until| Instant::now() >= until)
        .unwrap_or(true);
    let cognition_included = if cognition_config.enabled
        && cognition_allowed
        && let Some(cognition) = fetch_cognition_heartbeat_payload(client, cognition_config).await
        && let Some(obj) = payload.as_object_mut()
    {
        obj.insert("cognition".into(), cognition);
        true
    } else {
        false
    };
    if let Some(progress_json) = task_progress_payload.clone()
        && let Some(obj) = payload.as_object_mut()
    {
        obj.insert("task_progress".into(), progress_json);
    }
    HeartbeatPayload {
        payload,
        base_payload,
        task_progress_payload,
        cognition_included,
    }
}
