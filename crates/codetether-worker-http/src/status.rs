//! Worker identity and heartbeat status response.
use super::*;

/// Worker status endpoint - returns detailed worker state
pub(super) async fn worker_status(
    State(state): State<WorkerServerState>,
) -> Json<WorkerStatusResponse> {
    let connected = *state.connected.lock().await;
    let worker_id = state.worker_id.lock().await.clone();
    let heartbeat_state = state
        .internal_heartbeat
        .lock()
        .await
        .clone()
        .or_else(|| state.heartbeat_state.clone());

    let heartbeat_info = if let Some(ref hb_state) = heartbeat_state {
        let status: WorkerStatus = *hb_state.status.lock().await;
        let task_count = hb_state.active_task_count.lock().await;
        Some(HeartbeatInfo {
            status: status.as_str().to_string(),
            active_tasks: *task_count,
            agent_name: hb_state.agent_name.clone(),
        })
    } else {
        None
    };

    Json(WorkerStatusResponse {
        connected,
        worker_id,
        heartbeat: heartbeat_info,
    })
}
