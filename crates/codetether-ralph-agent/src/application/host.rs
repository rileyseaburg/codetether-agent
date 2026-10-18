//! Build Ralph prompts and tools using the existing application registry.
use super::{ApplicationAgent, execute};
use crate::{Host, Invocation};
use codetether_application_registry::{Application, ToolRegistry};
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::agent_retry::AgentOutcome;
use std::{path::Path, sync::Arc};
impl<A, H> Host for ApplicationAgent<A, H>
where
    A: Application,
    H: codetether_swarm_agent::Host<Registry = ToolRegistry<A>>,
{
    type Registry = ToolRegistry<A>;
    fn system_prompt(workspace: &Path) -> String {
        codetether_agent_core::builtin::build_system_prompt(workspace)
    }
    fn registry(provider: Arc<dyn Provider>, model: String) -> Arc<ToolRegistry<A>> {
        ToolRegistry::with_provider_arc(provider, model)
    }
    fn definitions(registry: &ToolRegistry<A>) -> Vec<ToolDefinition> {
        registry.definitions()
    }
    async fn execute(request: Invocation<'_, ToolRegistry<A>>) -> AgentOutcome {
        execute::run::<H>(request).await
    }
}
