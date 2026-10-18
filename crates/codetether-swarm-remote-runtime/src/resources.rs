//! Resources retained for the lifetime of a remote task.
use codetether_provider_core::{Provider, ToolDefinition};
use std::sync::Arc;

/// Provider registry, chosen provider, and workspace-specific tools.
///
/// The registry remains alive until result emission completes.
/// # Examples
/// ```no_run
/// # fn example<R, T>(registry: R, provider: std::sync::Arc<dyn codetether_provider_core::Provider>, tools: std::sync::Arc<T>) {
/// let resources = codetether_swarm_remote_runtime::Resources {
///     _registry: registry, provider, tool_registry: tools, tool_defs: vec![],
/// };
/// assert!(resources.tool_defs.is_empty());
/// # }
/// ```
pub struct Resources<R, T> {
    /// Retained provider registry, including any application-owned services.
    pub _registry: R,
    /// Provider selected by the payload.
    pub provider: Arc<dyn Provider>,
    /// Executable tool registry scoped to the workspace and capabilities.
    pub tool_registry: Arc<T>,
    /// Definitions advertised to the provider.
    pub tool_defs: Vec<ToolDefinition>,
}
