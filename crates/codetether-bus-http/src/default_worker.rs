//! Select an unambiguous active worker bus.
use super::*;

/// Resolve the default worker bus connection when the control plane has exactly
/// one active worker advertising a first-class bus/http interface.
///
/// # Arguments
/// The control-plane base URL, lookup identity when applicable, and optional bearer token.
///
/// # Returns
/// The resolved endpoint or workspace identity.
///
/// # Errors
/// Returns an error for failed HTTP requests, invalid responses or an unresolved assignment.
///
/// # Examples
/// ```rust,no_run
/// async fn lookup() -> anyhow::Result<()> {
///     let _result = codetether_bus_http::resolve_default_worker_bus_url("http://localhost", None).await?;
///     Ok(())
/// }
/// ```
pub async fn resolve_default_worker_bus_url(
    control_plane_url: &str,
    token: Option<&str>,
) -> anyhow::Result<String> {
    let workers_url = format!(
        "{}/v1/agent/workers",
        control_plane_url.trim_end_matches('/')
    );

    let client = reqwest::Client::new();
    let mut req = client.get(workers_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let workers: Vec<serde_json::Value> = req.send().await?.error_for_status()?.json().await?;

    let candidates = active_workers(workers);

    match candidates.as_slice() {
        [worker] => {
            let worker_id = worker
                .get("worker_id")
                .and_then(|value| value.as_str())
                .ok_or_else(|| anyhow::anyhow!("Active worker is missing worker_id"))?;
            resolve_worker_bus_url(control_plane_url, worker_id, token).await
        }
        [] => anyhow::bail!(
            "No active workers with first-class interfaces were found; deploy/register a worker or provide --worker-id"
        ),
        workers => {
            let worker_ids = workers
                .iter()
                .filter_map(|worker| worker.get("worker_id").and_then(|value| value.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!(
                "Multiple active workers are registered ({worker_ids}); provide --worker-id to choose one"
            )
        }
    }
}
