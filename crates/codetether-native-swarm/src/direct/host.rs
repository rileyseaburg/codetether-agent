//! Native bindings for the shared direct-swarm runtime.
use super::{Application, Host, ToolRegistry, providers, worker};
use codetether_swarm_core::result_store::ResultStore;
use codetether_swarm_tool::runtime::{LoopInput, SelectedProvider, WorkerContext};
use codetether_swarm_tool::task_input::TaskInput;
use std::sync::Arc;
impl<T: Application> codetether_swarm_tool::runtime::Host for Host<T> {
    type Registry = ToolRegistry<T>;
    type Observer = codetether_swarm_monitor::Observer;
    async fn load(
        requested: Option<&str>,
        observer: &mut Self::Observer,
    ) -> anyhow::Result<Result<SelectedProvider, codetether_tool_core::ToolResult>> {
        providers::load(requested, observer).await
    }
    fn registry(context: &WorkerContext, store: Arc<ResultStore>) -> Arc<ToolRegistry<T>> {
        worker::registry(context, store)
    }
    fn definitions(registry: &ToolRegistry<T>) -> Vec<codetether_provider_core::ToolDefinition> {
        registry.definitions()
    }
    fn system_prompt(context: &WorkerContext, task: &TaskInput) -> String {
        codetether_swarm_tool::prompt::application::build(
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
        input: LoopInput<ToolRegistry<T>>,
    ) -> anyhow::Result<(
        String,
        usize,
        usize,
        codetether_swarm_core::agent_retry::AgentLoopExit,
    )> {
        worker::run(input).await
    }
}
