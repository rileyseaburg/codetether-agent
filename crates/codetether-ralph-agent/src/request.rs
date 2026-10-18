//! Parameters passed unchanged to the native agent loop.
use codetether_bus_core::AgentBus;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::SwarmEvent;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
/// One prepared story or conflict-resolution invocation.
/// # Examples
/// Native hosts forward these fields to their existing loop without reinterpreting them.
pub struct Invocation<'a, R> {
    /// Selected provider.
    pub provider: Arc<dyn Provider>,
    /// Selected model.
    pub model: &'a str,
    /// Repository-aware system prompt.
    pub system: &'a str,
    /// Story/conflict request.
    pub prompt: &'a str,
    /// Filtered autonomous tool definitions.
    pub definitions: Vec<ToolDefinition>,
    /// Executable registry retained for the invocation.
    pub registry: Arc<R>,
    /// Maximum agent steps.
    pub max_steps: usize,
    /// Loop timeout in seconds.
    pub timeout_secs: u64,
    /// Optional lifecycle event sender.
    pub events: Option<mpsc::Sender<SwarmEvent>>,
    /// Story ID used by the loop (empty for conflict resolution).
    pub id: String,
    /// Optional local event bus.
    pub bus: Option<Arc<AgentBus>>,
    /// Repository root for execution.
    pub workspace: PathBuf,
}
