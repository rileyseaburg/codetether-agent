//! Worker registration request handling.

use anyhow::Result;
use reqwest::Client;

use super::{provider_models, registration_data, resolve_and_log_workspace_ids};
use codetether_a2a_worker_core::{advertised_interfaces, worker_capabilities};

pub async fn register_worker(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
    public_url: Option<&str>,
) -> Result<()> {
    let models = provider_models::load_provider_models()
        .await
        .unwrap_or_default();
    let workspace_ids = resolve_and_log_workspace_ids(client, server, token, codebases).await;
    let (hostname, k8s_node_name) = registration_data::host_metadata();
    let mut request = client.post(format!("{server}/v1/agent/workers/register"));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request
        .json(&serde_json::json!({
            "worker_id": worker_id, "name": name, "capabilities": worker_capabilities(), "hostname": hostname,
            "k8s_node_name": k8s_node_name, "models": registration_data::build_models_array(&models), "workspaces": codebases,
            "workspace_ids": workspace_ids, "interfaces": advertised_interfaces(public_url), "agents": registration_data::builtin_agent_defs(),
        }))
        .send()
        .await?;
    if response.status().is_success() {
        tracing::info!("Worker registered successfully");
    } else {
        tracing::warn!(status = %response.status(), "Failed to register worker");
    }
    Ok(())
}
