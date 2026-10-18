use crate::types::{TaskTriggerResponse, TriggerRequest};
use anyhow::{Context, Result};
use reqwest::Client;
pub(crate) async fn submit(
    client: &Client,
    server: &str,
    token: Option<&str>,
    workspace_id: &str,
    request: &TriggerRequest,
) -> Result<TaskTriggerResponse> {
    let url = format!(
        "{}/v1/agent/workspaces/{}/trigger",
        server,
        urlencoding::encode(&workspace_id)
    );
    let mut req = client.post(url).json(&request);
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let response = req
        .send()
        .await
        .context("Failed to reach CodeTether server")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Task trigger failed ({}): {}", status, body);
    }

    let payload: TaskTriggerResponse = response
        .json()
        .await
        .context("Failed to parse task trigger response")?;

    Ok(payload)
}
