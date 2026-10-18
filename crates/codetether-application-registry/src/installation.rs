//! Install an application default catalog while retaining its plugin registry.
use crate::{Application, ToolRegistry};
use codetether_plugin_verification::PluginRegistry;
use std::sync::Arc;
impl<H: Application> ToolRegistry<H> {
    /// Construct default tools, retain their plugins and pass ownership to a runtime.
    /// # Arguments
    /// `install` receives the shared tool registry after its plugins are cloned.
    /// # Returns
    /// The plugin registry belonging to the installed tool catalog.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_application_registry::Application>() {
    /// let mut installed = None;
    /// let _plugins = codetether_application_registry::ToolRegistry::<H>::install_defaults(|tools| installed = Some(tools));
    /// assert!(installed.is_some());
    /// # }
    /// ```
    pub fn install_defaults(install: impl FnOnce(Arc<Self>)) -> PluginRegistry {
        let runtime_tools = Arc::new(Self::with_defaults());
        let plugin_registry = runtime_tools.plugins().clone();
        install(runtime_tools);
        plugin_registry
    }
}
