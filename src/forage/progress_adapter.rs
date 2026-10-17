//! Translate application execution evidence into shared OKR progress updates.
use super::*;

pub(super) async fn record_execution_success_to_okr(
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
