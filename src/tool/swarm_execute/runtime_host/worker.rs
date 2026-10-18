//! Native restricted registry and provider-loop calls.
use crate::tool::ToolRegistry;
use codetether_swarm_core::result_store::ResultStore;
use codetether_swarm_tool::runtime::{LoopInput, WorkerContext};
use std::sync::Arc;
pub(super) fn registry(context: &WorkerContext, store: Arc<ResultStore>) -> Arc<ToolRegistry> {
    super::super::agent_registry::shared(
        context.read_only,
        context.verification,
        context.expects_changes,
        &context.working_dir,
        store,
        context.task_id.clone(),
        Arc::clone(&context.provider),
        context.model.clone(),
    )
}
pub(super) async fn run(
    input: LoopInput<ToolRegistry>,
) -> anyhow::Result<(String, usize, usize, crate::swarm::executor::AgentLoopExit)> {
    let context = input.context;
    crate::swarm::executor::run_agent_loop(
        context.provider,
        &context.model,
        &input.system,
        &input.user,
        input.tools,
        input.registry,
        input.max_steps,
        input.timeout_secs,
        Some(input.events),
        context.task_id,
        None,
        Some(context.working_dir),
    )
    .await
}
