//! Translate application execution evidence into shared OKR progress updates.
use super::*;

/// Record a completed execution's concrete evidence against its key result.
/// # Arguments
/// * `repo` — Repository receiving the progress update.
/// * `item` — Opportunity identifying the objective and key result.
/// * `args` — Engine and model used by the run.
/// * `execution_outcome` — Detail, changed files and quality result.
/// * `cycle` — Cycle number associated with this execution.
/// # Returns
/// Completes when the shared progress update has been persisted.
/// # Errors
/// Forwards objective lookup, validation and persistence failures.
/// # Examples
/// ```no_run
/// # async fn record(repo: &codetether_okr::OkrRepository, item: &codetether_forage_core::ForageOpportunity, args: &codetether_cli_options::ForageArgs, outcome: &codetether_forage_core::execution::ExecutionOutcome) -> anyhow::Result<()> {
/// codetether_forage_runtime::record_execution_success_to_okr(repo, item, args, outcome, 1).await?;
/// # Ok(()) }
/// ```
pub async fn record_execution_success_to_okr(
    repo: &OkrRepository,
    item: &ForageOpportunity,
    args: &ForageArgs,
    execution_outcome: &ExecutionOutcome,
    cycle: usize,
) -> Result<()> {
    let evidence = codetether_forage_core::progress::ExecutionEvidence {
        engine: &args.execution_engine,
        model: args.model.as_deref(),
        detail: &execution_outcome.detail,
        changed_files: &execution_outcome.changed_files,
        quality_gates_passed: execution_outcome.quality_gates_passed,
    };
    codetether_forage_core::progress::record(repo, item, &evidence, cycle).await
}
