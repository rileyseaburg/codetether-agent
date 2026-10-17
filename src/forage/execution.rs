//! Dispatch application engines and collect their execution evidence.
use super::*;

pub(super) async fn execute_opportunity(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<ExecutionOutcome> {
    let before = snapshot_git_changed_files();
    let detail = match args.execution_engine.as_str() {
        "swarm" => execute_opportunity_with_swarm(item, args).await?,
        "go" => execute_opportunity_with_go(item, args).await?,
        _ => execute_opportunity_with_run(item, args).await?,
    };
    let after = snapshot_git_changed_files();
    let changed_files = codetether_forage_core::execution::changed_files(before, after);

    // Run quality gates to verify the changes work
    let quality_result = run_quality_gates(&changed_files).await;

    Ok(codetether_forage_core::execution::with_quality(
        detail,
        changed_files,
        quality_result,
    ))
}

pub(super) async fn run_quality_gates(changed_files: &[String]) -> Result<(String, bool)> {
    codetether_forage_core::quality::run(changed_files, quality_shell::run).await
}
