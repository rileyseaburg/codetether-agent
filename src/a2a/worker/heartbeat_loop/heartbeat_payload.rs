use std::sync::Arc;

use reqwest::Client;
use tokio::{sync::Mutex, time::Instant};

use super::super::{
    CognitionHeartbeatConfig, HeartbeatState, WorkerStatus, fetch_cognition_heartbeat_payload,
};

pub(super) struct HeartbeatPayload {
    pub payload: serde_json::Value,
    pub base_payload: serde_json::Value,
    pub task_progress_payload: Option<serde_json::Value>,
    pub cognition_included: bool,
}

pub(super) async fn build_heartbeat_payload(
    client: &Client,
    heartbeat_state: &HeartbeatState,
    cognition_config: &CognitionHeartbeatConfig,
    task_progress: &Arc<Mutex<crate::a2a::worker::task_timeline::TaskProgressState>>,
    active_count: usize,
    cognition_disabled_until: Option<Instant>,
) -> HeartbeatPayload {
    heartbeat_state.set_task_count(active_count).await;
    heartbeat_state
        .set_status(if active_count > 0 {
            WorkerStatus::Processing
        } else {
            WorkerStatus::Idle
        })
        .await;
    let status = heartbeat_state.status.lock().await.as_str().to_string();
    let sub_agents = heartbeat_state.sub_agents_snapshot().await;
    let progress = task_progress.lock().await.clone();
    let task_progress_payload = progress.task_id.as_ref().map(|_| serde_json::json!({ "task_id": progress.task_id, "current_checkpoint": progress.current_checkpoint, "elapsed_secs": format!("{:.1}", progress.elapsed_secs), "remaining_secs": format!("{:.1}", progress.remaining_secs), "budget_pct_used": format!("{:.1}%", progress.budget_pct_used), "checkpoints_reached": progress.checkpoints_reached.len(), "last_detail": progress.last_detail }));
    let base_payload = serde_json::json!({ "worker_id": &heartbeat_state.worker_id, "agent_name": &heartbeat_state.agent_name, "status": status, "active_task_count": active_count, "sub_agents": sub_agents });
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
