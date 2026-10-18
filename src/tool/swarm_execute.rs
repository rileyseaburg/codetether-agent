//! Direct-swarm tool bound to application worker execution and TUI observation.
pub(crate) use codetether_swarm_tool::prompt::application as agent_prompt;
pub(crate) mod agent_registry;
mod provider_setup;
mod runtime_host;
pub(crate) use codetether_swarm_tool::support;
pub(crate) mod tui_bridge;

/// Shared swarm tool using the application's provider, registry and observer.
///
/// # Examples
/// ```
/// use codetether_agent::tool::swarm_execute::SwarmExecuteTool;
/// let _tool = SwarmExecuteTool::new();
/// ```
pub type SwarmExecuteTool = codetether_swarm_tool::SwarmExecuteTool<runtime_host::Native>;
