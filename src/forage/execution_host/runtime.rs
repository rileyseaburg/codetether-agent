//! Delegate prepared engine requests to existing application entry points.
use super::{Native, NativeSwarm};
use crate::{
    cli::RunArgs,
    okr::{Okr, OkrRun},
    provider::{Provider, ProviderRegistry},
    swarm::SwarmConfig,
};
use codetether_go_ralph::GoRalphResult;
use std::sync::Arc;
impl codetether_forage_execution::Host for Native {
    type Registry = ProviderRegistry;
    type Swarm = NativeSwarm;
    async fn run(args: RunArgs) -> anyhow::Result<()> {
        crate::cli::run::execute(args).await
    }
    fn swarm(config: SwarmConfig) -> NativeSwarm {
        NativeSwarm::new(config)
    }
    async fn providers() -> anyhow::Result<ProviderRegistry> {
        ProviderRegistry::from_vault().await
    }
    async fn go(
        task: &str,
        okr: &mut Okr,
        run: &mut OkrRun,
        provider: Arc<dyn Provider>,
        model: &str,
        max_iterations: usize,
        max_concurrent: usize,
    ) -> anyhow::Result<GoRalphResult> {
        crate::cli::go_ralph::execute_go_ralph(
            task,
            okr,
            run,
            provider,
            model,
            max_iterations,
            None,
            max_concurrent,
            None,
        )
        .await
    }
    async fn evaluate_tool_invocation(
        name: &str,
        args: &serde_json::Value,
    ) -> Option<crate::tool::ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(name, args).await
    }
}
