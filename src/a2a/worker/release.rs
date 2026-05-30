//! Worker task release client.

pub(super) use super::release_payload::Payload;
use anyhow::{Context, Result};
use reqwest::Client;

pub(super) async fn send(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: Payload<'_>,
) -> Result<()> {
    let response = request(client, server, token, worker_id, payload)
        .send()
        .await
        .context("Failed to send task release request")?;
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }
    let body = response.text().await.unwrap_or_default();
    anyhow::bail!("Task release failed with HTTP {status}: {body}")
}

fn request(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: Payload<'_>,
) -> reqwest::RequestBuilder {
    let req = client.post(format!("{server}/v1/worker/tasks/release"));
    let req = req.header("X-Worker-ID", worker_id).json(&payload);
    match token {
        Some(token) => req.bearer_auth(token),
        None => req,
    }
}
