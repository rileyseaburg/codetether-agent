//! Worker registration metadata assembly and signed submission.
use anyhow::Result;
use codetether_a2a_worker_core::{advertised_interfaces, registration, worker_capabilities};
use codetether_provider_core::ModelInfo;
use reqwest::Client;
use std::{collections::HashMap, future::Future};

/// Resolve registration metadata and submit the canonical worker payload.
/// # Arguments
/// * `client`, `server`, and `token` — Existing control-plane connection.
/// * `worker_id` and `name` — Worker identity used in payloads and signatures.
/// * `codebases` — Workspace paths advertised by the worker.
/// * `public_url` — Optional public interface advertised for incoming tasks.
/// * `models` — Application provider discovery; errors produce an empty catalog.
/// # Returns
/// Unit after the registration request is accepted.
/// # Errors
/// Returns serialization, signing, HTTP transport, or registration rejection errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_a2a_http::worker::register_worker(
///     &reqwest::Client::new(), "http://localhost:8001", &None,
///     "worker", "agent", &[], None,
///     async { Ok(std::collections::HashMap::new()) },
/// ).await?;
/// # Ok(()) }
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn register_worker(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
    public_url: Option<&str>,
    models: impl Future<Output = Result<HashMap<String, Vec<ModelInfo>>>>,
) -> Result<()> {
    let models = models.await.unwrap_or_default();
    let workspace_ids =
        crate::workspace::resolve_and_log_workspace_ids(client, server, token, codebases).await;
    let (hostname, k8s_node_name) = registration::host_metadata();
    let payload = serde_json::json!({
        "worker_id": worker_id, "name": name, "capabilities": worker_capabilities(), "hostname": hostname,
        "k8s_node_name": k8s_node_name, "models": registration::build_models_array(&models), "workspaces": codebases,
        "workspace_ids": workspace_ids, "interfaces": advertised_interfaces(public_url), "agents": registration::builtin_agent_defs(),
    });
    super::register_worker_payload(client, server, token, worker_id, name, &payload).await
}
