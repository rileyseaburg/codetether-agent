//! Resolve cached providers and build workspace-scoped verifier tools.
use super::{ApplicationHost, execute, prompt};
use crate::runtime::{Host, Run};
use codetether_application_providers::{Application as Providers, ProviderRegistry};
use codetether_application_registry::{Application as Tools, ToolRegistry};
use codetether_provider_core::{Provider, ToolDefinition};
use codetether_swarm_core::agent_retry::AgentLoopExit;
use std::{path::Path, sync::Arc};

impl<P, T, A, W> Host for ApplicationHost<P, T, A, W>
where
    P: Providers,
    T: Tools,
    A: codetether_swarm_agent::Host<Registry = ToolRegistry<T>>,
    W: codetether_workspace_tools::composition::Factory<Registry = ToolRegistry<T>>,
{
    type Providers = Arc<ProviderRegistry<P>>;
    type Tools = ToolRegistry<T>;
    async fn resolve(
        requested: &str,
    ) -> anyhow::Result<(Self::Providers, Arc<dyn Provider>, String)> {
        let providers = ProviderRegistry::<P>::shared_from_vault().await?;
        let (provider, model) = providers.resolve_model(requested)?;
        Ok((providers, provider, model))
    }
    fn tools(workspace: &Path, provider: &Arc<dyn Provider>, model: &str) -> Arc<ToolRegistry<T>> {
        codetether_workspace_tools::composition::direct::standard::<W>(
            false,
            true,
            workspace,
            Arc::clone(provider),
            model.to_string(),
        )
    }
    fn definitions(tools: &ToolRegistry<T>) -> Vec<ToolDefinition> {
        tools.definitions()
    }
    fn system_prompt(
        workspace: &Path,
        model: &str,
        claimed: codetether_goal_core::GoalStatus,
    ) -> String {
        prompt::system_prompt(workspace, model, claimed)
    }
    async fn execute(
        run: Run<'_, ToolRegistry<T>>,
    ) -> anyhow::Result<(String, usize, usize, AgentLoopExit)> {
        execute::run::<T, A>(run).await
    }
}
