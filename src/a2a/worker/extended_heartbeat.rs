//! Signed request adapter for shared extended-heartbeat payloads.
#[path = "extended_heartbeat_request.rs"]
mod request;
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
    let (task_id, payload) = codetether_a2a_http::worker::prepare_extended_heartbeat(
        worker_id,
        progress,
        lease_seconds,
    )?;
    let response = request::send(client, server, token, worker_id, &task_id, &payload).await?;
    codetether_a2a_http::worker::check_extended_heartbeat(response, &task_id).await
}
