//! Retain the isolated workspace while executing and reporting one PRD.
use super::BenchmarkRunner;
use crate::{provider::Provider, ralph::Prd};
use codetether_benchmark::{PrdBenchmarkResult, pricing::ModelPricing};
use std::{path::Path, sync::Arc};
use tracing::info;
impl BenchmarkRunner {
    pub(super) async fn run_single_prd(
        &self,
        prd_path: &Path,
        tier: u8,
        provider: Arc<dyn Provider>,
        model: &str,
        pricing: &Option<ModelPricing>,
    ) -> anyhow::Result<PrdBenchmarkResult> {
        let prd = Prd::load(&prd_path.to_path_buf()).await?;
        let prd_id = prd_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let prd_feature = prd.feature.clone();
        let prd_quality_checks = prd.quality_checks.clone();

        // Scaffold an isolated workspace (temp dir with cargo init + git init)
        let (_tmp_handle, workspace_prd_path) = Self::scaffold_workspace(prd_path, &prd)?;
        let workspace_dir: &Path = workspace_prd_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Invalid workspace PRD path"))?;

        info!(
            "Benchmark workspace ready at {:?} for PRD '{}'",
            workspace_dir, prd_feature
        );

        let (_ralph_loop, state, measurements) = self
            .execute_prd(&workspace_prd_path, provider, model, pricing)
            .await?;
        Ok(Self::finish_prd(
            &state,
            prd_id,
            tier,
            prd_feature,
            measurements,
            workspace_dir,
            &prd_quality_checks,
        ))
    }
}
