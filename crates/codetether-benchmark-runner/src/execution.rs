//! Run Ralph between the original timing and token snapshot boundaries.
use super::{
    Application, BenchmarkRunner,
    measurements::{self, Measurements},
};
use codetether_benchmark::pricing::ModelPricing;
use codetether_provider_core::Provider;
use codetether_ralph_core::RalphState;
use codetether_telemetry::TOKEN_USAGE;
use std::{path::PathBuf, sync::Arc, time::Instant};
impl<H: Application> BenchmarkRunner<H> {
    pub(super) async fn execute_prd(
        &self,
        workspace_prd_path: &PathBuf,
        provider: Arc<dyn Provider>,
        model: &str,
        pricing: &Option<ModelPricing>,
    ) -> anyhow::Result<(H::Loop, RalphState, Measurements)> {
        // Snapshot token usage before
        let tokens_before = TOKEN_USAGE.global_snapshot();

        let start = Instant::now();

        let config = self.ralph_config(workspace_prd_path);
        // RalphLoop::new reads the PRD and uses its parent as working_dir
        let mut ralph_loop = H::start(
            workspace_prd_path.clone(),
            provider,
            model.to_string(),
            config,
        )
        .await?;

        let state = H::run(&mut ralph_loop).await?;
        let duration = start.elapsed();

        let measurements = measurements::finish(tokens_before, duration, pricing);
        Ok((ralph_loop, state, measurements))
    }
}
