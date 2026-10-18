//! Native base prompt and tool-registry constructors for Ralph agents.
use crate::{provider::Provider, tool::ToolRegistry};
use codetether_ralph_agent::{Host, Invocation};
use std::{path::Path, sync::Arc};
mod execute;
/// Native execution binding exposed through the stage host associated type.
/// # Examples
/// ```rust,no_run
/// type Task = <codetether_agent::ralph::RalphLoop as codetether_ralph_stage::Host>::Task;
/// type Binding = <Task as codetether_ralph_task::Host>::Agent;
/// fn assert_host<H: codetether_ralph_agent::Host>() {}
/// assert_host::<Binding>();
/// ```
pub struct Native;
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
