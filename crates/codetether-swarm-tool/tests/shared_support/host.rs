//! Bind the real scheduler and prompt builder to the test catalog.
use super::{TestHost, observer::Sink, registry::Catalog};
use codetether_swarm_core::result_store::ResultStore;
use codetether_swarm_tool::{runtime::*, task_input::TaskInput};
use std::sync::Arc;
impl Host for TestHost {
    type Registry = Catalog;
    type Observer = Sink;
    async fn load(
        _: Option<&str>,
        _: &mut Sink,
    ) -> anyhow::Result<Result<SelectedProvider, codetether_tool_core::ToolResult>> {
        Ok(Ok(super::provider::selected()))
    }
    fn registry(context: &WorkerContext, _: Arc<ResultStore>) -> Arc<Catalog> {
        let capability = codetether_workspace_tools::Capability::from_task_flags(
            context.read_only,
            context.verification,
            context.expects_changes,
        );
        assert!(capability.is_mutating());
        Arc::new(codetether_workspace_tools::finish::<Self>(
            Catalog::default(),
            &context.working_dir,
            capability,
        ))
    }
    fn definitions(registry: &Catalog) -> Vec<codetether_provider_core::ToolDefinition> {
        super::registry::definitions(registry)
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
            context.shared_workspace,
        )
    }
    async fn run(input: LoopInput<Catalog>) -> codetether_swarm_core::agent_retry::AgentOutcome {
        super::run::execute(input).await
    }
}
