//! Save the generated PRD before starting the objective run.

use anyhow::{Context, Result};
use codetether_okr::OkrRun;
use codetether_ralph_core::Prd;
use std::path::PathBuf;
pub(crate) async fn save(prd: &Prd, okr_run: &OkrRun) -> Result<(String, PathBuf)> {
    let prd_filename = format!("prd_{}.json", okr_run.id.to_string().replace('-', "_"));
    let prd_path = PathBuf::from(&prd_filename);
    prd.save(&prd_path)
        .await
        .context("Failed to save generated PRD")?;

    tracing::info!(target: "codetether_agent::cli::go_ralph",
        prd_path = %prd_path.display(),
        stories = prd.user_stories.len(),
        feature = %prd.feature,
        "PRD generated and saved"
    );

    Ok((prd_filename, prd_path))
}
