//! Resolve the bus endpoint advertised by one worker.
use super::*;

/// Resolve a worker's first-class bus connection via the control plane.
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
///     let _result = codetether_bus_http::resolve_worker_bus_url("http://localhost", "worker", None).await?;
///     Ok(())
/// }
/// ```
pub async fn resolve_worker_bus_url(
    control_plane_url: &str,
    worker_id: &str,
    token: Option<&str>,
) -> anyhow::Result<String> {
    let worker_url = format!(
        "{}/v1/agent/workers/{}",
        control_plane_url.trim_end_matches('/'),
        worker_id
    );

    let client = reqwest::Client::new();
    let mut req = client.get(worker_url);
    if let Some(token) = token.filter(|value| !value.trim().is_empty()) {
        req = req.bearer_auth(token);
    }

    let worker: serde_json::Value = req.send().await?.error_for_status()?.json().await?;

    let has_bus_interface = worker
        .get("interfaces")
        .and_then(|value| value.get("bus"))
        .and_then(|value| value.get("stream_url"))
        .and_then(|value| value.as_str())
        .is_some();
    let has_http_interface = worker
        .get("interfaces")
        .and_then(|value| value.get("http"))
        .and_then(|value| value.get("base_url"))
        .and_then(|value| value.as_str())
        .is_some();

    if has_bus_interface || has_http_interface {
        return Ok(format!(
            "{}/v1/agent/workers/{}/bus/stream",
            control_plane_url.trim_end_matches('/'),
            worker_id
        ));
    }

    anyhow::bail!(
        "Worker '{}' does not advertise a first-class bus interface",
        worker_id
    )
}
