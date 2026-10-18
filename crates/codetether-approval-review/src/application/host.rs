//! Resolve reviewer resources and map requests to the shared agent loop.
use super::{Application, Reviewer, read_only_tools};
use crate::{Host, Run};
use codetether_application_providers::ProviderRegistry;
use codetether_application_registry::ToolRegistry;
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::agent_retry::AgentLoopExit;
use std::sync::Arc;
impl<H: Application> Host for Reviewer<H> {
    type Registry = ProviderRegistry<H::Providers>;
    type Tools = ToolRegistry<H::Tools>;
    fn resolve(
        registry: &Self::Registry,
        selector: &str,
    ) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        registry.resolve_model(selector)
    }
    fn tools() -> Arc<Self::Tools> {
        Arc::new(read_only_tools::<H::Tools>())
    }
    fn definitions(tools: &Self::Tools) -> Vec<ToolDefinition> {
        tools.definitions()
    }
    async fn execute(
        run: Run<'_, Self::Tools>,
    ) -> anyhow::Result<(String, usize, usize, AgentLoopExit)> {
        codetether_swarm_agent::run_agent_loop::<H::Agent>(
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
