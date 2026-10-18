//! Plugin discovery wire response.
use codetether_plugin_verification::PluginManifest;
use serde::Serialize;
/// Server signing availability and registered plugin manifests.
/// # Examples
/// ```
/// let response = codetether_plugin_http::PluginListResponse {
///     server_fingerprint: "server".into(), signing_available: false, plugins: vec![],
/// };
/// assert!(response.plugins.is_empty());
/// ```

#[derive(Serialize)]
pub struct PluginListResponse {
    pub server_fingerprint: String,
    pub signing_available: bool,
    pub plugins: Vec<PluginManifest>,
}
