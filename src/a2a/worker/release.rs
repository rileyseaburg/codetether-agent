//! Worker task release client.

pub(super) use super::release_payload::Payload;
use anyhow::Result;
use reqwest::Client;
pub(super) async fn send(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: Payload<'_>,
) -> Result<()> {
    codetether_a2a_http::worker::send_release(request(client, server, token, worker_id, payload)?)
        .await
}

fn request(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    payload: Payload<'_>,
) -> Result<reqwest::RequestBuilder> {
    let req = client.post(format!("{server}/v1/worker/tasks/release"));
    let req = req.header("X-Worker-ID", worker_id).json(&payload);
    let resource = super::worker_security::mutation_resource(payload.task_id, &payload)?;
    let req = match token {
        Some(token) => req.bearer_auth(token),
        None => req,
    };
    super::worker_security::apply_configured_proof(req, "release", worker_id, &resource)
}
