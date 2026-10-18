//! Start a forage run while retaining ownership of its live services.
use super::*;

/// Run forage using the configured selection and execution options.
///
/// # Arguments
/// `args` supplies goal inputs, execution choices and loop limits.
/// # Returns
/// Completes when the configured run finishes.
/// # Errors
/// Returns setup, discovery or execution errors according to the fail-fast policy.
/// # Examples
/// ```rust,no_run
/// async fn example(args: codetether_agent::cli::ForageArgs) -> anyhow::Result<()> {
///     let _ = codetether_agent::forage::execute(args).await?;
///     Ok(())
/// }
/// ```
pub async fn execute(args: ForageArgs) -> Result<()> {
    execute_with_summary(args).await.map(|_| ())
}

/// Run forage using the configured selection and execution options.
///
/// # Arguments
/// `args` supplies goal inputs, execution choices and loop limits.
/// # Returns
/// The run counters and final selection.
/// # Errors
/// Returns setup, discovery or execution errors according to the fail-fast policy.
/// # Examples
/// ```rust,no_run
/// async fn example(args: codetether_agent::cli::ForageArgs) -> anyhow::Result<()> {
///     let _ = codetether_agent::forage::execute_with_summary(args).await?;
///     Ok(())
/// }
/// ```
pub async fn execute_with_summary(args: ForageArgs) -> Result<ForageRunSummary> {
    codetether_forage_runtime::execute_with_summary::<super::execution_host::Native>(args).await
}
