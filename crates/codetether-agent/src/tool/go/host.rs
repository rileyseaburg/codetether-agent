//! Provider-discovery and Ralph-executor bindings for the autonomous tool.
use crate::{
    bus::AgentBus,
    cli::go_ralph::{GoRalphResult, execute_go_ralph},
    okr::{Okr, OkrRun},
    provider::{Provider, ProviderRegistry},
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
/// Application provider registry and Ralph executor.
pub struct AppHost;
#[async_trait]
impl codetether_go_tool::Host for AppHost {
    type Registry = ProviderRegistry;
    type Output = GoRalphResult;
    async fn registry() -> Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    async fn execute(
        task: &str,
        okr: &mut Okr,
        run: &mut OkrRun,
        provider: Arc<dyn Provider>,
        model: &str,
        max_iterations: usize,
        bus: Option<Arc<AgentBus>>,
        max_concurrent: usize,
        registry: Option<Arc<ProviderRegistry>>,
    ) -> Result<GoRalphResult> {
        execute_go_ralph(
            task,
            okr,
            run,
            provider,
            model,
            max_iterations,
            bus,
            max_concurrent,
            registry,
        )
        .await
    }
    fn summarize(result: GoRalphResult, task: &str) -> codetether_go_tool::Completion {
        super::result::summarize(result, task)
    }
}
