//! Construction policy for shared sub-agent execution state.
use super::{AgentState, Conversation};
use crate::SwarmEvent;
use codetether_bus_core::AgentBus;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_session_control::ControlHandle;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;

impl<R> AgentState<R> {
    /// Initialize a sub-agent conversation and capture the current parent control.
    /// # Arguments
    /// * `provider` and `model` — Completion backend and effective model.
    /// * `system` and `user` — Initial conversation messages.
    /// * `tools` and `registry` — Advertised definitions and executable registry.
    /// * `max_steps` and `timeout_secs` — Turn and wall-clock limits.
    /// * `events`, `subtask_id`, and `bus` — Observability and task identity.
    /// * `working_dir` — Optional workspace for tool execution.
    /// # Returns
    /// Fresh state with empty counters/output, a deadline, model temperature,
    /// and the parent control active at construction time.
    /// # Examples
    /// ```no_run
    /// # fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
    /// let state = codetether_swarm_core::agent_loop::AgentState::new(
    ///     provider, "model", "system", "user", vec![], std::sync::Arc::new(()),
    ///     10, 60, None, "task".into(), None, None,
    /// );
    /// assert_eq!(state.conversation.steps, 0);
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        provider: Arc<dyn Provider>,
        model: &str,
        system: &str,
        user: &str,
        tools: Vec<ToolDefinition>,
        registry: Arc<R>,
        max_steps: usize,
        timeout_secs: u64,
        events: Option<mpsc::Sender<SwarmEvent>>,
        subtask_id: String,
        bus: Option<Arc<AgentBus>>,
        working_dir: Option<PathBuf>,
    ) -> Self {
        let model = model.to_string();
        let mut conversation = Conversation::new(system, user, timeout_secs);
        conversation.temperature = codetether_provider_policy::temperature_for(&model);
        Self {
            provider,
            model,
            tools,
            registry,
            max_steps,
            timeout_secs,
            events,
            subtask_id,
            bus,
            working_dir,
            conversation,
            control: ControlHandle::current(),
        }
    }
}
