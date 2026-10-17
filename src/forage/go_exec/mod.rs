//! Application Go-engine preparation, execution and presentation.
//!
//! The executor retains provider and repository ownership across Ralph
//! execution; helper modules prepare those dependencies and render results.
use super::*;
use crate::okr::{OkrRun, OkrRunStatus};
use crate::provider::ProviderRegistry;
use anyhow::Context;
mod okr;
mod provider;
mod render;

/// Execute opportunity using the Ralph PRD-driven autonomous loop (go engine)
pub(super) async fn execute_opportunity_with_go(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<String> {
    use crate::cli::go_ralph::execute_go_ralph;

    let timeout_secs = args.run_timeout_secs.clamp(30, 86_400);
    let (_registry, provider, resolved_model) = provider::prepare(args).await?;

    // Load the OKR to get full context for PRD generation
    let (_repo, mut okr, mut okr_run) = okr::prepare(item).await?;

    // Build the task from the opportunity
    let task = item.prompt.clone();

    // Execute the Ralph PRD-driven autonomous loop
    match tokio::time::timeout(
        Duration::from_secs(timeout_secs),
        execute_go_ralph(
            &task,
            &mut okr,
            &mut okr_run,
            provider,
            &resolved_model,
            10,   // max_iterations
            None, // bus - could pass bus here for inter-iteration learning
            3,    // max_concurrent_stories
            None, // registry - passed via RalphLoop.with_registry if needed
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
