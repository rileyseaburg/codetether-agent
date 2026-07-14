//! Public entry point for the reusable sub-agent loop.

use super::super::AgentLoopExit;
use crate::tui::swarm_view::SwarmEvent;
use crate::{
    bus::AgentBus,
    provider::{Provider, ToolDefinition},
    tool::ToolRegistry,
};
use anyhow::Result;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;

/// Runs a deadline-bound sub-agent conversation with tool execution.
///
/// # Arguments
///
/// * `provider` and `model` — Completion backend and model identifier.
/// * `system` and `user` — Initial conversation prompts.
/// * `tools` and `registry` — Advertised definitions and executable tools.
/// * `max_steps` and `timeout_secs` — Safety limits for the loop.
/// * `events`, `subtask_id`, and `bus` — Runtime observability channels.
/// * `working_dir` — Optional filesystem root enforced for tool calls.
///
/// # Returns
///
/// Accumulated text, step count, tool-call count, and terminal exit reason.
///
/// # Errors
///
/// Returns an error when the provider request or agent setup fails.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::{provider::Provider, swarm::run_agent_loop, tool::ToolRegistry};
/// use std::sync::Arc;
/// # fn prepare(provider: Arc<dyn Provider>, registry: Arc<ToolRegistry>) {
/// let future = run_agent_loop(
///     provider, "model", "Be concise.", "Inspect the project.", Vec::new(),
///     registry, 10, 60, None, "task-1".into(), None, None,
/// );
/// drop(future);
/// # }
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn run_agent_loop(
    provider: Arc<dyn Provider>,
    model: &str,
    system: &str,
    user: &str,
    tools: Vec<ToolDefinition>,
    registry: Arc<ToolRegistry>,
    max_steps: usize,
    timeout_secs: u64,
    events: Option<mpsc::Sender<SwarmEvent>>,
    subtask_id: String,
    bus: Option<Arc<AgentBus>>,
    working_dir: Option<PathBuf>,
) -> Result<(String, usize, usize, AgentLoopExit)> {
    let mut state = super::create::state(
        provider,
        model,
        system,
        user,
        tools,
        registry,
        max_steps,
        timeout_secs,
        events,
        subtask_id,
        bus,
        working_dir,
    );
    let exit = super::driver::execute(&mut state).await?;
    Ok((state.output, state.steps, state.tool_calls, exit))
}
