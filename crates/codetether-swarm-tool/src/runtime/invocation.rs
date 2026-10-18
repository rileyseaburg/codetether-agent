//! Assemble and run a worker after its concurrency permit is acquired.
use super::{Host, LoopInput, WorkerContext, messages};
use crate::task_input::TaskInput;
use codetether_swarm_core::{SwarmEvent, agent_retry::AgentLoopExit, result_store::ResultStore};
use std::sync::Arc;
use tokio::sync::mpsc;
pub(super) async fn run<H: Host>(
    task: &TaskInput,
    context: WorkerContext,
    store: Arc<ResultStore>,
    events: mpsc::Sender<SwarmEvent>,
    max_steps: usize,
    timeout_secs: u64,
) -> anyhow::Result<(String, usize, usize, AgentLoopExit)> {
    messages::started(&events, &context, task);
    let registry = H::registry(&context, store);
    let tools = H::definitions(&registry);
    let system = H::system_prompt(&context, task);
    let user = messages::user(task);
    let (output, steps, tool_calls, exit) = H::run(LoopInput {
        context,
        system,
        user,
        tools,
        registry,
        max_steps,
        timeout_secs,
        events,
    })
    .await?;

    Ok((output, steps, tool_calls, exit))
}
