//! Native bindings for the shared direct-swarm runtime.
use super::{provider_setup, tui_bridge};
use crate::tool::ToolRegistry;
use codetether_swarm_core::result_store::ResultStore;
use codetether_swarm_tool::runtime::{Host, LoopInput, SelectedProvider, WorkerContext};
use codetether_swarm_tool::task_input::TaskInput;
use std::sync::Arc;
mod observer;
mod worker;
pub(super) struct Native;
impl Host for Native {
    type Registry = ToolRegistry;
    type Observer = tui_bridge::Observer;
    async fn load(
        requested: Option<&str>,
        observer: &mut Self::Observer,
    ) -> anyhow::Result<Result<SelectedProvider, crate::tool::ToolResult>> {
        provider_setup::load(requested, observer).await
    }
    fn registry(context: &WorkerContext, store: Arc<ResultStore>) -> Arc<ToolRegistry> {
        worker::registry(context, store)
    }
    fn definitions(registry: &ToolRegistry) -> Vec<crate::provider::ToolDefinition> {
        registry.definitions()
    }
    fn system_prompt(context: &WorkerContext, task: &TaskInput) -> String {
        super::agent_prompt::build(
            &context.task_id,
            task.specialty.as_deref(),
            &context.working_dir,
            &context.model,
            &task.instruction,
            context.read_only,
            context.expects_changes,
        )
    }
    async fn run(
        input: LoopInput<ToolRegistry>,
    ) -> anyhow::Result<(String, usize, usize, crate::swarm::executor::AgentLoopExit)> {
        worker::run(input).await
    }
}
