//! Preserve verifier request fields when invoking the shared agent loop.
use crate::runtime::Run;
use codetether_application_registry::{Application, ToolRegistry};
use codetether_swarm_core::agent_retry::AgentLoopExit;

pub(super) async fn run<
    T: Application,
    A: codetether_swarm_agent::Host<Registry = ToolRegistry<T>>,
>(
    request: Run<'_, ToolRegistry<T>>,
) -> anyhow::Result<(String, usize, usize, AgentLoopExit)> {
    codetether_swarm_agent::run_agent_loop::<A>(
        request.provider,
        request.model,
        request.system,
        request.user,
        request.definitions,
        request.tools,
        request.max_steps,
        request.timeout_secs,
        None,
        request.id,
        None,
        Some(request.workspace),
    )
    .await
}
