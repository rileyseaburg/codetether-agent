//! Native provider initialization and Ralph-loop operations for benchmarks.
use crate::provider::{Provider, ProviderRegistry};
use crate::ralph::{RalphConfig, RalphLoop, RalphState};
use anyhow::Result;
use std::{path::PathBuf, sync::Arc};
/// Native services selected by the public benchmark runner alias.
pub struct Native;
impl codetether_benchmark_runner::Application for Native {
    type Registry = ProviderRegistry;
    type Loop = RalphLoop;
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    async fn registry() -> Result<ProviderRegistry> {
        ProviderRegistry::from_vault().await
    }
    fn provider(registry: &ProviderRegistry, name: &str) -> Option<Arc<dyn Provider>> {
        registry.get(name)
    }
    async fn start(
        path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
    ) -> Result<RalphLoop> {
        RalphLoop::new(path, provider, model, config).await
    }
    async fn run(loop_state: &mut RalphLoop) -> Result<RalphState> {
        loop_state.run().await
    }
}
