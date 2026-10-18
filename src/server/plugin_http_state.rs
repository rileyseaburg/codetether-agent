//! Native plugin registry and server identity bindings for HTTP discovery.
impl codetether_plugin_http::http::HttpState for super::AppState {
    fn plugin_registry(&self) -> &codetether_plugin_verification::PluginRegistry {
        &self.inner.plugin_registry
    }
    fn server_fingerprint(&self) -> &str {
        &self.inner.server_fingerprint
    }
}
