//! Native plugin registry and server identity bindings for HTTP discovery.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_plugin_http::http::HttpState for AppState<R> {
    fn plugin_registry(&self) -> &codetether_plugin_verification::PluginRegistry {
        &self.inner.plugin_registry
    }
    fn server_fingerprint(&self) -> &str {
        &self.inner.server_fingerprint
    }
}
