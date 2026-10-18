//! Executes a forage opportunity through `codetether run`.
//!
//! Builds the [`RunArgs`] for a single opportunity prompt and runs it with a
//! clamped timeout, translating outcomes into forage-facing result strings.

use crate::Host;
use anyhow::Result;
use codetether_cli_options::{ForageArgs, RunArgs};
use codetether_forage_core::ForageOpportunity;
use std::time::Duration;

/// Runs `item`'s prompt via the `run` CLI path with a bounded timeout.
///
/// # Errors
///
/// Returns an error if run execution fails or the clamped timeout
/// (30s..=86400s from `args.run_timeout_secs`) elapses.
pub(crate) async fn execute_opportunity_with_run<H: Host>(
    item: &ForageOpportunity,
    args: &ForageArgs,
) -> Result<String> {
    let run_args = RunArgs {
        message: item.prompt.clone(),
        continue_session: false,
        session: None,
        model: args.model.clone(),
        agent: Some("build".to_string()),
        access_mode: None,
        yolo: false,
        format: "default".to_string(),
        file: Vec::new(),
        codex_session: None,
        max_steps: None,
        auto_continue_until: None,
        branches: 1,
        strategies: Vec::new(),
        sol_planner: false,
    };
    let timeout_secs = args.run_timeout_secs.clamp(30, 86_400);
    match tokio::time::timeout(Duration::from_secs(timeout_secs), H::run(run_args)).await {
        Ok(Ok(())) => Ok("run execution completed".to_string()),
        Ok(Err(err)) => Err(err),
        Err(_) => anyhow::bail!("run execution timed out after {timeout_secs}s"),
    }
}
