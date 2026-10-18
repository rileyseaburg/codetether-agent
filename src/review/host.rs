//! Native provider and read-only tool bindings for approval review.
use crate::{provider::ProviderRegistry, tool::ToolRegistry};
use codetether_approval_review::{Host, Run};
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::agent_retry::AgentLoopExit;
use std::sync::Arc;
pub(super) struct Native;
impl Host for Native {
    type Registry = ProviderRegistry;
    type Tools = ToolRegistry;
    fn resolve(
        registry: &ProviderRegistry,
        selector: &str,
    ) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        registry.resolve_model(selector)
    }
    fn tools() -> Arc<ToolRegistry> {
        Arc::new(super::tools::read_only_tools())
    }
    fn definitions(tools: &ToolRegistry) -> Vec<ToolDefinition> {
        tools.definitions()
    }
    async fn execute(
        run: Run<'_, ToolRegistry>,
    ) -> anyhow::Result<(String, usize, usize, AgentLoopExit)> {
        crate::swarm::run_agent_loop(
            run.provider,
            run.model,
            run.system,
            run.user,
            run.definitions,
            run.tools,
            run.max_steps,
            run.timeout_secs,
            None,
            run.id,
            None,
            Some(run.workspace),
        )
        .await
    }
}
