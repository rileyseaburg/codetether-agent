//! Native base prompt and tool-registry constructors for Ralph agents.
use super::*;
use codetether_ralph_agent::{Host, Invocation};
mod execute;
pub(in crate::ralph::ralph_loop) struct Native;
impl Host for Native {
    type Registry = ToolRegistry;
    fn system_prompt(workspace: &Path) -> String {
        crate::agent::builtin::build_system_prompt(workspace)
    }
    fn registry(provider: Arc<dyn Provider>, model: String) -> Arc<ToolRegistry> {
        ToolRegistry::with_provider_arc(provider, model)
    }
    fn definitions(registry: &ToolRegistry) -> Vec<crate::provider::ToolDefinition> {
        registry.definitions()
    }
    async fn execute(
        request: Invocation<'_, ToolRegistry>,
    ) -> codetether_swarm_core::agent_retry::AgentOutcome {
        execute::run(request).await
    }
}
