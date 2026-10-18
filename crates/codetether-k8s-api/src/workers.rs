//! Worker status inferred from pod names and phases.
use axum::{Json, http::StatusCode};

// ── Worker connectivity ─────────────────────────────────────────────────

/// List connected workers (K8s pods with agent label)
/// # Arguments
/// * `manager` - Manager supplying pod observations.
/// # Returns
/// Worker-shaped pod records; failed listing produces an empty worker list.
/// # Errors
/// This compatibility projection currently always returns Ok.
/// # Examples
/// ```no_run
/// # async fn example(manager: &codetether_k8s::K8sManager) {
/// let _ = codetether_k8s_api::list_connected_workers(manager).await;
/// # }
/// ```
pub async fn list_connected_workers(
    manager: &codetether_k8s::K8sManager,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Use K8s pod listing as proxy for connected workers
    let pods = manager.list_pods().await.unwrap_or_default();
    let workers: Vec<serde_json::Value> = pods
        .iter()
        .filter(|p| p.name.contains("codetether") || p.name.contains("worker"))
        .map(|p| {
            serde_json::json!({
                "worker_id": p.name,
                "name": p.name,
                "status": p.phase,
                "is_sse_connected": p.phase == "Running",
                "last_seen": chrono::Utc::now().to_rfc3339(),
            })
        })
        .collect();
    Ok(Json(serde_json::json!({ "workers": workers })))
}
