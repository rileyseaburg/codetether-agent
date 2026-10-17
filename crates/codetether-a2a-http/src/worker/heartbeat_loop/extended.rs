//! Compose shared extended-heartbeat preparation, transport, and response handling.
use anyhow::Result;
use reqwest::Client;
pub(super) async fn send_extended_task_heartbeat(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    progress: serde_json::Value,
    lease_seconds: u64,
) -> Result<()> {
    let (task_id, payload) =
        super::super::prepare_extended_heartbeat(worker_id, progress, lease_seconds)?;
    let response = super::super::extended_heartbeat::send(
        client, server, token, worker_id, &task_id, &payload,
    )
    .await?;
    super::super::check_extended_heartbeat(response, &task_id).await
}
