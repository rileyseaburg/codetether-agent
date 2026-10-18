//! Native provider and tool-registry bindings for observed verification.
mod execute;
use crate::{
    provider::{Provider, ProviderRegistry, ToolDefinition},
    tool::ToolRegistry,
};
use codetether_goal_verification::runtime::Run;
use std::{path::Path, sync::Arc};
pub(super) struct Native;
impl codetether_goal_verification::runtime::Host for Native {
    type Providers = Arc<ProviderRegistry>;
    type Tools = ToolRegistry;
    async fn resolve(
        requested: &str,
    ) -> anyhow::Result<(Self::Providers, Arc<dyn Provider>, String)> {
        let providers = ProviderRegistry::shared_from_vault().await?;
        let (provider, model) = providers.resolve_model(requested)?;
        Ok((providers, provider, model))
    }
    fn tools(workspace: &Path, provider: &Arc<dyn Provider>, model: &str) -> Arc<ToolRegistry> {
        crate::tool::swarm_execute::agent_registry::standard(
            false,
            true,
            workspace,
            Arc::clone(provider),
            model.to_string(),
        )
    }
    fn definitions(tools: &ToolRegistry) -> Vec<ToolDefinition> {
        tools.definitions()
    }
    fn system_prompt(
        workspace: &Path,
        model: &str,
        claimed: crate::session::tasks::GoalStatus,
    ) -> String {
        super::prompt::system_prompt(workspace, model, claimed)
    }
    async fn execute(
        run: Run<'_, ToolRegistry>,
    ) -> anyhow::Result<(String, usize, usize, crate::swarm::executor::AgentLoopExit)> {
        execute::run(run).await
    }
}
