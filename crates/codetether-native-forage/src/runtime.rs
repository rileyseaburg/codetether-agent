//! Native providers and policy with application-owned execution callbacks.
use super::*;
use codetether_native_providers::ProviderRegistry;
impl<E: Execution> codetether_forage_execution::Host for Host<E> {
    type Registry = ProviderRegistry;
    type Swarm = E::Swarm;
    async fn run(args: RunArgs) -> anyhow::Result<()> {
        E::run(args).await
    }
    fn swarm(config: SwarmConfig) -> Self::Swarm {
        E::swarm(config)
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
        E::go(
            task,
            okr,
            run,
            provider,
            model,
            max_iterations,
            max_concurrent,
        )
        .await
    }
    async fn evaluate_tool_invocation(
        name: &str,
        args: &serde_json::Value,
    ) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(name, args).await
    }
}
