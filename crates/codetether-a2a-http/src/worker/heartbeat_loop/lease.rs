//! Extend the active task lease after a worker heartbeat tick.
use super::*;
pub(super) async fn renew(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    persistent: bool,
    progress: Option<serde_json::Value>,
    lease: u64,
) {
    if persistent
        && let Some(progress_json) = progress
        && let Err(error) =
            send_extended_task_heartbeat(&client, &server, &token, worker_id, progress_json, lease)
                .await
    {
        tracing::warn!(target: "codetether_agent::a2a::worker::heartbeat_loop", worker_id = %worker_id, error = %error, "Extended task heartbeat failed");
    }
}
