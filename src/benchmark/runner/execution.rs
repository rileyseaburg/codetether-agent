//! Run Ralph between the original timing and token snapshot boundaries.
use super::{
    BenchmarkRunner,
    measurements::{self, Measurements},
};
use crate::{
    provider::Provider,
    ralph::{RalphLoop, RalphState},
    telemetry::TOKEN_USAGE,
};
use codetether_benchmark::pricing::ModelPricing;
use std::{path::PathBuf, sync::Arc, time::Instant};
impl BenchmarkRunner {
    pub(super) async fn execute_prd(
        &self,
        workspace_prd_path: &PathBuf,
        provider: Arc<dyn Provider>,
        model: &str,
        pricing: &Option<ModelPricing>,
    ) -> anyhow::Result<(RalphLoop, RalphState, Measurements)> {
        // Snapshot token usage before
        let tokens_before = TOKEN_USAGE.global_snapshot();

        let start = Instant::now();

        let config = self.ralph_config(workspace_prd_path);
        // RalphLoop::new reads the PRD and uses its parent as working_dir
        let mut ralph_loop = RalphLoop::new(
            workspace_prd_path.clone(),
            provider,
            model.to_string(),
            config,
        )
        .await?;

        let state = ralph_loop.run().await?;
        let duration = start.elapsed();

        let measurements = measurements::finish(tokens_before, duration, pricing);
        Ok((ralph_loop, state, measurements))
    }
}
