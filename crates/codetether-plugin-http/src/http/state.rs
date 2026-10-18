//! Application registry and identity access for plugin discovery.
use codetether_plugin_verification::PluginRegistry;

/// State supplying plugin discovery's registry and signing identity.
/// # Implementors
/// Implement this trait on the application's shared HTTP state.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Return the existing verified plugin registry.
    fn plugin_registry(&self) -> &PluginRegistry;
    /// Return the fingerprint used by the existing signing probe.
    fn server_fingerprint(&self) -> &str;
}
