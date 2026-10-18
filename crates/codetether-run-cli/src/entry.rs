//! Top-level Sol route and JSONL failure reporting.
use anyhow::Result;
use codetether_cli_options::RunArgs;

/// Execute one CLI run and preserve JSON Lines failure events.
/// # Arguments
/// * `args` - Parsed run-command options.
/// # Returns
/// Unit after the selected execution path finishes.
/// # Errors
/// Returns preparation, execution or output errors after JSONL failure reporting.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_run_cli::Host>(args: codetether_cli_options::RunArgs) -> anyhow::Result<()> where <H::Prd as codetether_go_ralph::Host>::Registry: codetether_autochat_core::model_rotation::ModelRegistry {
/// codetether_run_cli::execute::<H>(args).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: crate::Host>(args: RunArgs) -> Result<()>
where
    <H::Prd as codetether_go_ralph::Host>::Registry:
        codetether_autochat_core::model_rotation::ModelRegistry,
{
    let jsonl_output = codetether_run_jsonl::enabled(&args.format);
    let use_sol_planner = args.sol_planner;
    let result = if use_sol_planner {
        codetether_sol_workflow::execute::<H::Sol>(args).await
    } else {
        crate::normal::execute::<H>(args).await
    };
    if jsonl_output && let Err(error) = &result {
        codetether_run_jsonl::write_failed(error.to_string())?;
    }
    result
}
