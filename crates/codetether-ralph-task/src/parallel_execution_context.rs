//! Borrowed host inputs for one parallel-story model operation.
use super::*;
/// Borrowed model and relay inputs tied to the task's permit and bridge scope.
/// # Examples
/// ```rust,no_run
/// # use codetether_ralph_task::{Host, Task, Context};
/// # use codetether_swarm_core::SwarmEvent;
/// # use tokio::sync::mpsc::Sender;
/// # fn example<H: Host>(task: &Task<H>, bridge: &Option<Sender<SwarmEvent>>) {
/// let context = Context::<H> {
///     provider: &task.provider, model: &task.model, story: &task.story,
///     working_dir: &task.working_dir, ralph_tx: &task.ralph_tx,
///     bridge_tx: bridge, bus: &task.bus, registry: &task.registry,
///     delegation: task.delegation.as_ref(), relay_enabled: task.relay_enabled,
///     relay_max_agents: task.relay_max_agents, relay_max_rounds: task.relay_max_rounds,
///     max_steps_per_story: task.max_steps_per_story,
/// };
/// assert_eq!(context.story.id, task.story.id);
/// # }
/// ```
pub struct ParallelExecutionContext<'a, H: Host> {
    /// Selected model provider.
    pub provider: &'a Arc<dyn Provider>,
    /// Selected model name.
    pub model: &'a str,
    /// Story whose requirements and identity drive this task.
    pub story: &'a UserStory,
    /// Original workspace or selected worktree root.
    pub working_dir: &'a PathBuf,
    /// Optional story lifecycle event sender.
    pub ralph_tx: &'a Option<mpsc::Sender<RalphEvent>>,
    /// Borrowed sender for forwarding single-agent events to Ralph.
    pub bridge_tx: &'a Option<mpsc::Sender<SwarmEvent>>,
    /// Optional shared agent event bus.
    pub bus: &'a Option<Arc<AgentBus>>,
    /// Optional provider registry retained for relay execution.
    pub registry: &'a Option<Arc<H::Registry>>,
    /// Optional provider-selection calibration state.
    pub delegation: Option<&'a DelegationState>,
    /// Whether to select relay execution when a registry is available.
    pub relay_enabled: bool,
    /// Requested relay team size.
    pub relay_max_agents: usize,
    /// Requested relay round limit.
    pub relay_max_rounds: usize,
    /// Step budget for the initial single-agent execution.
    pub max_steps_per_story: usize,
}
