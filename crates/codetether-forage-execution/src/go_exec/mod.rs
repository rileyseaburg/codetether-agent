//! Application Go-engine preparation, execution and presentation.
//!
//! The executor retains provider and repository ownership across Ralph
//! execution; helper modules prepare those dependencies and render results.
use crate::*;
use std::time::Duration;

use codetether_okr::OkrRunStatus;

use anyhow::Context;
mod okr;
mod provider;
mod render;

/// Execute opportunity using the Ralph PRD-driven autonomous loop (go engine)
pub(crate) async fn execute_opportunity_with_go<H: Host>(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<String> {
    let timeout_secs = args.run_timeout_secs.clamp(30, 86_400);
    let (_registry, provider, resolved_model) = provider::prepare::<H>(args).await?;

    // Load the OKR to get full context for PRD generation
    let (_repo, mut okr, mut okr_run) = okr::prepare(item).await?;

    // Build the task from the opportunity
    let task = item.prompt.clone();

    // Execute the Ralph PRD-driven autonomous loop
    match tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        H::go(
            &task,
            &mut okr,
            &mut okr_run,
            provider,
            &resolved_model,
            10, // max_iterations
            3,  // max_concurrent_stories
        ),
    )
    .await
    {
        Ok(Ok(result)) => render::result(result),
        Ok(Err(err)) => {
            // Update run status to failed
            okr_run.status = OkrRunStatus::Failed;
            Err(err).context("go execution failed")
        }
        Err(_) => anyhow::bail!("go execution timed out after {timeout_secs}s"),
    }
}
