//! Forward each prepared Ralph invocation to the shared agent loop.
use crate::Invocation;
pub(super) async fn run<H: codetether_swarm_agent::Host>(
    request: Invocation<'_, H::Registry>,
) -> codetether_swarm_core::agent_retry::AgentOutcome {
    codetether_swarm_agent::run_agent_loop::<H>(
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
