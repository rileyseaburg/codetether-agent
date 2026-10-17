//! Refresh active count and status before snapshotting worker state.
use super::*;
pub(super) async fn snapshot(
    heartbeat_state: &HeartbeatState,
    active_count: usize,
) -> (String, Vec<String>) {
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
    (status, sub_agents)
}
