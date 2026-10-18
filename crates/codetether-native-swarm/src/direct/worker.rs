//! Native restricted registry and provider-loop calls.
use super::{Application, ToolRegistry};
use codetether_swarm_core::result_store::ResultStore;
use codetether_swarm_tool::runtime::{LoopInput, WorkerContext};
use std::sync::Arc;
pub(super) fn registry<T: Application>(
    context: &WorkerContext,
    store: Arc<ResultStore>,
) -> Arc<ToolRegistry<T>> {
    super::registry::shared::<T>(
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
pub(super) async fn run<T: Application>(
    input: LoopInput<ToolRegistry<T>>,
) -> anyhow::Result<(
    String,
    usize,
    usize,
    codetether_swarm_core::agent_retry::AgentLoopExit,
)> {
    let context = input.context;
    codetether_swarm_agent::run_agent_loop::<crate::AgentHost<T>>(
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
