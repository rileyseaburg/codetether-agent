use crate::Host;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_tool_core::Tool;
use std::sync::{Arc, Weak};
/// Native catalog factories and coordination bindings.
/// # Implementors
/// Applications supply their catalog, batch implementation, provider-backed
/// search construction, and weak registry binding. Workspace policy stays shared.
pub trait Factory: Host {
    /// Batch tool bound after the completed registry is shared.
    type Batch: Tool + 'static;
    /// Construct the application's default catalog.
    fn defaults() -> Self::Registry;
    /// Construct the application's provider-aware catalog.
    fn provider(provider: Arc<dyn Provider>, model: String) -> Self::Registry;
    /// Construct an initially unbound batch tool.
    fn batch() -> Arc<Self::Batch>;
    /// Bind batch weakly to its owning registry.
    fn bind_batch(batch: &Self::Batch, registry: Weak<Self::Registry>);
    /// Add routed search using the supplied provider.
    fn search(registry: &mut Self::Registry, provider: Arc<dyn Provider>);
    /// Collect definitions from the completed registry.
    fn definitions(registry: &Self::Registry) -> Vec<ToolDefinition>;
}
