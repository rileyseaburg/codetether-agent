//! Load the objective and create its approved forage run.
use crate::{ForageOpportunity, OkrRepository};
use anyhow::{Context, Result};
use codetether_okr::{Okr, OkrRun};
pub(super) async fn prepare(item: &ForageOpportunity) -> Result<(OkrRepository, Okr, OkrRun)> {
    let repo = OkrRepository::from_config()
        .await
        .context("Failed to load OKR repository")?;

    let okr = repo
        .get_okr(item.okr_id)
        .await?
        .with_context(|| format!("OKR {} not found", item.okr_id))?;

    // Create an OKR run for this execution
    let mut okr_run = OkrRun::new(item.okr_id, format!("forage-cycle-{}", item.key_result_id));
    okr_run.submit_for_approval()?;
    okr_run.record_decision(codetether_okr::ApprovalDecision::approve(
        okr_run.id,
        "Auto-approved from forage execution",
    ));

    Ok((repo, okr, okr_run))
}
