//! Stop over-budget requests and emit the original one-time warning.
use crate::{CostGuardStatus, check::check};

/// Enforce the cost budget before sending a provider request.
///
/// Returns `Err` if the hard limit has been reached. Logs a one-shot
/// warning the first time the warn threshold is crossed. Returns `Ok(())`
/// in the no-limits-configured case.
/// # Arguments
/// Takes no arguments; reads shared thresholds and accumulated provider spend.
/// # Returns
/// Ok when no hard limit has been reached.
/// # Errors
/// Returns the configured hard-limit message when current spend reaches it.
/// # Examples
/// ```no_run
/// # fn before_request() -> anyhow::Result<()> {
/// codetether_cost_guard::enforce_cost_budget()?;
/// # Ok(())
/// # }
/// ```
pub fn enforce_cost_budget() -> anyhow::Result<()> {
    match check() {
        CostGuardStatus::Block {
            spent_usd,
            limit_usd,
        } => {
            anyhow::bail!(
                "Cost guardrail tripped: session has spent ~${:.2} which meets/exceeds the \
                 hard limit of ${:.2}. Raise CODETETHER_COST_LIMIT_USD (or \
                 `[guardrails] hard_limit_usd` in config) to continue.",
                spent_usd,
                limit_usd
            )
        }
        CostGuardStatus::Warned {
            spent_usd,
            threshold_usd,
        } => {
            tracing::warn!(target: "codetether_agent::session::helper::cost_guard",
                spent_usd,
                threshold_usd,
                "Cost guardrail warn threshold reached; set CODETETHER_COST_LIMIT_USD to cap spend"
            );
            Ok(())
        }
        CostGuardStatus::Ok => Ok(()),
    }
}
