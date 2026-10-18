//! Public entry point for the reusable sub-agent loop.

use crate::AgentLoopExit;
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::SwarmEvent;
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
/// ```no_run
/// # fn example<H: codetether_swarm_agent::Host>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>, registry: std::sync::Arc<H::Registry>) {
/// let future = codetether_swarm_agent::run_agent_loop::<H>(
///     provider, "model", "system", "task", vec![], registry,
///     10, 60, None, "subtask".into(), None, None,
/// );
/// drop(future);
/// # }
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn run_agent_loop<H: crate::Host>(
    provider: Arc<dyn Provider>,
    model: &str,
    system: &str,
    user: &str,
    tools: Vec<ToolDefinition>,
    registry: Arc<H::Registry>,
    max_steps: usize,
    timeout_secs: u64,
    events: Option<mpsc::Sender<SwarmEvent>>,
    subtask_id: String,
    bus: Option<Arc<AgentBus>>,
    working_dir: Option<PathBuf>,
) -> Result<(String, usize, usize, AgentLoopExit)> {
    let mut state =
        super::state::State::<H>::from(codetether_swarm_core::agent_loop::AgentState::new(
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
        ));
    let exit = super::driver::execute(&mut state).await?;
    Ok((
        state.inner.conversation.output,
        state.inner.conversation.steps,
        state.inner.conversation.tool_calls,
        exit,
    ))
}
