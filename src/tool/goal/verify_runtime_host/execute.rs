//! Forward the shared verification request to the unchanged native agent loop.
use crate::{
    swarm::executor::{AgentLoopExit, run_agent_loop},
    tool::ToolRegistry,
};
use codetether_goal_verification::runtime::Run;
pub(super) async fn run(
    request: Run<'_, ToolRegistry>,
) -> anyhow::Result<(String, usize, usize, AgentLoopExit)> {
    run_agent_loop(
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
