//! Signed request transport for extended task heartbeats.

use anyhow::Result;
use reqwest::{Client, Response};

pub(super) async fn send(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task_id: &str,
    payload: &serde_json::Value,
) -> Result<Response> {
    let request = client
        .post(format!("{server}/v1/worker/tasks/heartbeat-extended"))
        .json(payload);
    let resource = super::super::worker_security::mutation_resource(task_id, payload)?;
    let mut request = super::super::worker_security::apply_configured_proof(
        request,
        "heartbeat-extended",
        worker_id,
        &resource,
    )?;
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    Ok(request.send().await?)
}
