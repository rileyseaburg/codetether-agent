//! Dispatch application engines and collect their execution evidence.
use crate::*;
use codetether_forage_core::changes::snapshot_git_changed_files;

/// Execute one opportunity and collect its change and quality evidence.
/// # Arguments
/// * `item` — Selected opportunity and prompt.
/// * `args` — Engine selection, model and execution limits.
/// # Returns
/// Engine detail, changed files and quality-check evidence.
/// # Errors
/// Forwards engine preparation, execution and timeout errors.
/// # Examples
/// ```no_run
/// # async fn run<H: codetether_forage_execution::Host>(item: &codetether_forage_core::ForageOpportunity, args: &codetether_cli_options::ForageArgs) -> anyhow::Result<()> {
/// let outcome = codetether_forage_execution::execute_opportunity::<H>(item, args).await?;
/// # let _ = outcome;
/// # Ok(()) }
/// ```
pub async fn execute_opportunity<H: Host>(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<ExecutionOutcome> {
    let before = snapshot_git_changed_files();
    let detail = match args.execution_engine.as_str() {
        "swarm" => execute_opportunity_with_swarm::<H>(item, args).await?,
        "go" => execute_opportunity_with_go::<H>(item, args).await?,
        _ => execute_opportunity_with_run::<H>(item, args).await?,
    };
    let after = snapshot_git_changed_files();
    let changed_files = codetether_forage_core::execution::changed_files(before, after);

    // Run quality gates to verify the changes work
    let quality_result = run_quality_gates::<H>(&changed_files).await;

    Ok(codetether_forage_core::execution::with_quality(
        detail,
        changed_files,
        quality_result,
    ))
}

/// Run quality gates through the application's authorization policy.
/// # Arguments
/// * `changed_files` — Changed paths used to select applicable checks.
/// # Returns
/// Combined check output and whether all applicable checks passed.
/// # Errors
/// The shared quality runner records individual command failures in its output.
/// # Examples
/// ```no_run
/// # async fn check<H: codetether_forage_execution::Host>() -> anyhow::Result<()> {
/// let (_, passed) = codetether_forage_execution::run_quality_gates::<H>(&[]).await?;
/// assert!(passed);
/// # Ok(()) }
/// ```
pub async fn run_quality_gates<H: Host>(changed_files: &[String]) -> Result<(String, bool)> {
    codetether_forage_core::quality::run(changed_files, quality_shell::run::<H>).await
}
