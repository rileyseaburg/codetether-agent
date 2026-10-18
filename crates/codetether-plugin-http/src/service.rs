//! Plugin discovery response construction for an existing registry.
use crate::PluginListResponse;
use axum::Json;
use codetether_plugin_verification::PluginRegistry;

/// List registered plugins.
/// # Arguments
/// * `registry` - Existing verified plugin registry.
/// * `server_fingerprint` - Server identity included in the response and probe.
/// # Returns
/// Registered manifests and the result of the existing signing probe.
/// # Examples
/// ```no_run
/// # async fn example(registry: &codetether_plugin_verification::PluginRegistry) {
/// let _ = codetether_plugin_http::list_plugins(registry, "server").await;
/// # }
/// ```
pub async fn list_plugins(
    registry: &PluginRegistry,
    server_fingerprint: &str,
) -> Json<PluginListResponse> {
    let test_sig = registry
        .signing_key()
        .sign("_probe", "0.0.0", server_fingerprint);
    Json(PluginListResponse {
        server_fingerprint: server_fingerprint.to_owned(),
        signing_available: !test_sig.is_empty(),
        plugins: registry.list().await,
    })
}
