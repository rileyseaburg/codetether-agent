//! Shared sub-agent execution state with an application-supplied tool registry.
use super::Conversation;
use crate::SwarmEvent;
use codetether_bus_core::AgentBus;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_session_control::ControlHandle;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;

/// Sub-agent services, configuration, control, and conversation ownership.
///
/// The registry parameter keeps application tool construction outside the
/// shared state. Native adapters can wrap this value to implement their hooks.
/// # Examples
/// ```no_run
/// # fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// use codetether_swarm_core::agent_loop::{AgentState, Conversation};
/// let state = AgentState {
///     provider, model: "model".into(), tools: vec![],
///     registry: std::sync::Arc::new(()), max_steps: 1, timeout_secs: 60,
///     events: None, subtask_id: "task".into(), bus: None, working_dir: None,
///     conversation: Conversation::new("system", "user", 60), control: None,
/// };
/// assert_eq!(state.conversation.steps, 0);
/// # }
/// ```
pub struct AgentState<R> {
    /// Completion provider for this agent.
    pub provider: Arc<dyn Provider>,
    /// Effective model identifier.
    pub model: String,
    /// Tools advertised to the provider.
    pub tools: Vec<ToolDefinition>,
    /// Application registry used to resolve executable tools.
    pub registry: Arc<R>,
    /// Maximum provider-turn count.
    pub max_steps: usize,
    /// Configured execution time limit in seconds.
    pub timeout_secs: u64,
    /// Optional live swarm event channel.
    pub events: Option<mpsc::Sender<SwarmEvent>>,
    /// Subtask identity used in events and tool context.
    pub subtask_id: String,
    /// Optional shared agent message bus.
    pub bus: Option<Arc<AgentBus>>,
    /// Optional isolated workspace root.
    pub working_dir: Option<PathBuf>,
    /// Mutable conversation and progress state.
    pub conversation: Conversation,
    /// Optional parent-session control handle.
    pub control: Option<ControlHandle>,
}
