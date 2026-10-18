//! Forward the prepared Ralph invocation to the native agent loop.
use crate::tool::ToolRegistry;
use codetether_ralph_agent::Invocation;
pub(super) async fn run(
    request: Invocation<'_, ToolRegistry>,
) -> codetether_swarm_core::agent_retry::AgentOutcome {
    crate::swarm::run_agent_loop(
        request.provider,
        request.model,
        request.system,
        request.prompt,
        request.definitions,
        request.registry,
        request.max_steps,
        request.timeout_secs,
        request.events,
        request.id,
        request.bus,
        Some(request.workspace),
    )
    .await
}
