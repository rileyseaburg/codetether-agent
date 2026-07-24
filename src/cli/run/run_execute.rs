//! Top-level routing for normal and Sol-planned CLI runs.

use anyhow::Result;

use super::{RunArgs, execute_inner, jsonl, sol_workflow};

/// Execute one CLI run and preserve JSON Lines failure events.
pub async fn execute(args: RunArgs) -> Result<()> {
    let jsonl_output = jsonl::enabled(&args.format);
    let use_sol_planner = args.sol_planner;
    let result = if use_sol_planner {
        sol_workflow::execute(args).await
    } else {
        execute_inner(args).await
    };
    if jsonl_output && let Err(error) = &result {
        jsonl::write_failed(error.to_string())?;
    }
    result
}
